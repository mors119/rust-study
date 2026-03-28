use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{Duration, sleep};
use tokio_tungstenite::{accept_async, connect_async, tungstenite::protocol::Message};

/*
필요한 dependencies

cargo add tokio --features=full
cargo add tokio-tungstenite
cargo add futures-util
cargo add serde --features=derive
cargo add serde_json
cargo add chrono

역할 요약
- tokio: 비동기 runtime, TCP 소켓, task 실행
- tokio-tungstenite: WebSocket handshake + stream 처리
- futures-util: send(), next(), split() 같은 stream/sink 확장 메서드
- serde: Rust struct <-> 직렬화/역직렬화
- serde_json: JSON 문자열 변환
- chrono: 시간 정보 생성
*/

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    sender: String,
    content: String,
    sent_at: String,
}

impl ChatMessage {
    fn new(sender: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            sender: sender.into(),
            content: content.into(),
            sent_at: Utc::now().to_rfc3339(),
        }
    }
}

// TCP 연결을 받은 뒤, 핸드셰이크로 웹소켓 스트림으로 바꾸고, 그 스트림에서 JSON 메시지를 주고받는 구조
#[tokio::main]
pub async fn run() {
    // 서버를 백그라운드 task로 실행한다.
    // 실무에서는 보통 main에서 서버만 띄우지만,
    // 학습용으로는 client까지 같이 돌려보는 편이 흐름을 이해하기 쉽다.
    let server_handle = tokio::spawn(async {
        if let Err(e) = run_server("127.0.0.1:8080").await {
            eprintln!("server error: {e}");
        }
    });

    // 서버가 먼저 bind를 끝내고 listen 상태가 되도록 잠깐 대기
    sleep(Duration::from_millis(200)).await;

    // 예제용 클라이언트 실행
    if let Err(e) = run_client("ws://127.0.0.1:8080").await {
        eprintln!("client error: {e}");
    }

    // 학습용 예제이므로 마지막에 서버 task를 종료한다.
    // 실제 서비스 서버는 abort하지 않고 계속 실행 상태를 유지한다.
    server_handle.abort();
}

async fn run_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 1) 일반 TCP 서버 소켓을 연다.
    // 웹소켓도 처음에는 TCP 연결에서 시작한다.
    let listener = TcpListener::bind(addr).await?;
    println!("server listening on {addr}");

    loop {
        // 2) 클라이언트의 TCP 연결을 기다린다.
        let (stream, peer_addr) = listener.accept().await?;
        println!("new tcp connection: {peer_addr}");

        // 3) 클라이언트마다 별도 task에서 처리한다.
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, peer_addr.to_string()).await {
                eprintln!("connection error ({peer_addr}): {e}");
            }
        });
    }
}

async fn handle_connection(
    stream: TcpStream,
    peer_addr: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // 4) TCP stream을 WebSocket stream으로 승격한다.
    // 여기서 웹소켓 handshake(승인 과정)가 처리된다.
    let ws_stream = accept_async(stream).await?;
    println!("websocket handshake completed: {peer_addr}");

    // 5) 하나의 WebSocket stream을 송신/수신으로 분리한다.
    // sender: send() 담당
    // receiver: next() 담당
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // 6) 연결 성공 후 환영 메시지 전송
    let welcome = ChatMessage::new("server", format!("{peer_addr} connected. welcome!"));

    let welcome_json = serde_json::to_string(&welcome)?;
    ws_sender.send(Message::Text(welcome_json.into())).await?;

    // 7) 클라이언트가 보내는 메시지를 계속 수신한다.
    while let Some(msg_result) = ws_receiver.next().await {
        match msg_result {
            Ok(Message::Text(text)) => {
                // 클라이언트가 보낸 JSON 문자열을 ChatMessage로 파싱
                match serde_json::from_str::<ChatMessage>(&text) {
                    Ok(client_msg) => {
                        println!(
                            "server received from {}: {}",
                            client_msg.sender, client_msg.content
                        );

                        // echo 응답 생성
                        let response =
                            ChatMessage::new("server", format!("echo: {}", client_msg.content));

                        let response_json = serde_json::to_string(&response)?;
                        ws_sender.send(Message::Text(response_json.into())).await?;
                    }
                    Err(e) => {
                        eprintln!("invalid json from client: {e}");
                    }
                }
            }

            Ok(Message::Close(frame)) => {
                println!("client requested close: {frame:?}");
                break;
            }

            Ok(other) => {
                // 텍스트 외의 메시지(binary, ping, pong 등)는 여기서 확인 가능
                println!("server received non-text message: {other:?}");
            }

            Err(e) => {
                eprintln!("websocket receive error: {e}");
                break;
            }
        }
    }

    println!("connection closed: {peer_addr}");
    Ok(())
}

async fn run_client(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 1) connect_async로 서버에 웹소켓 연결 요청
    // 보통 &str URL을 넘기면 된다.
    let (ws_stream, response) = connect_async(url).await?;
    println!("client connected, http status: {}", response.status());

    // 2) stream을 송신/수신으로 분리
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // 3) 서버로부터 오는 메시지를 처리하는 task
    let recv_handle = tokio::spawn(async move {
        while let Some(msg_result) = ws_receiver.next().await {
            match msg_result {
                Ok(Message::Text(text)) => match serde_json::from_str::<ChatMessage>(&text) {
                    Ok(msg) => {
                        println!(
                            "client received => sender={}, content={}, sent_at={}",
                            msg.sender, msg.content, msg.sent_at
                        );
                    }
                    Err(e) => {
                        eprintln!("client failed to parse json: {e}");
                    }
                },
                Ok(Message::Close(frame)) => {
                    println!("client received close frame: {frame:?}");
                    break;
                }
                Ok(other) => {
                    println!("client received non-text message: {other:?}");
                }
                Err(e) => {
                    eprintln!("client receive error: {e}");
                    break;
                }
            }
        }
    });

    // 4) 클라이언트 메시지 전송
    let outgoing_messages = ["hello server", "this is websocket over tokio", "goodbye"];

    for content in outgoing_messages {
        let chat_msg = ChatMessage::new("rust-client", content);
        let json = serde_json::to_string(&chat_msg)?;

        println!("client sending: {content}");
        ws_sender.send(Message::Text(json.into())).await?;

        // 너무 빠르게 보내면 로그가 정신없을 수 있어서 예제용으로 약간 대기
        sleep(Duration::from_millis(300)).await;
    }

    // 5) 연결 종료 프레임 전송
    // 클라이언트가 명시적으로 close를 보내면 서버도 종료 흐름을 타기 쉽다.
    ws_sender.send(Message::Close(None)).await?;

    // 수신 task 종료 대기
    let _ = recv_handle.await;

    Ok(())
}
