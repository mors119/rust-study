use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{Duration, sleep};

#[tokio::main]
pub async fn run() -> io::Result<()> {
    // 서버를 백그라운드 task로 실행한다.
    // bind + accept 루프는 오래 살아 있는 작업이므로 spawn으로 분리하는 것이 자연스럽다.
    let server_handle = tokio::spawn(async {
        if let Err(err) = run_server("127.0.0.1:8080").await {
            eprintln!("server error: {err}");
        }
    });

    // 서버가 bind를 끝내고 listen 상태가 될 시간을 아주 잠깐 준다.
    // 학습용 데모에서는 이렇게 해도 되지만,
    // 실무에서는 readiness 신호(channel 등)로 동기화하는 편이 더 좋다.
    sleep(Duration::from_millis(100)).await;

    // 테스트용 클라이언트 실행
    run_client("127.0.0.1:8080", "hello tokio tcp").await?;

    // 예제에서는 서버 task를 계속 살려둘 필요가 없으므로 abort 한다.
    // 실제 서버 애플리케이션에서는 보통 abort하지 않고 프로세스가 계속 떠 있다.
    server_handle.abort();

    Ok(())
}

// TCP 서버를 실행한다.
// 하나의 listener로 여러 클라이언트를 계속 받아들인다.
async fn run_server(addr: &str) -> io::Result<()> {
    // 지정한 주소와 포트에 TCP 서버 소켓을 바인딩한다. 이제 이 포트로 들어오는 TCP 연결을 받을 수 있다.
    let listener = TcpListener::bind(addr).await?;
    println!("server listening on {addr}");

    loop {
        // 새 클라이언트가 접속할 때까지 비동기적으로 기다린다.
        // 반환값:
        // - socket: 해당 클라이언트와 통신할 개별 TcpStream
        // - peer_addr: 접속한 상대 주소
        let (socket, peer_addr) = listener.accept().await?;
        println!("new client connected: {peer_addr}");

        // 클라이언트마다 별도 task를 만들어 동시에 처리한다.
        // 이게 없으면 한 클라이언트를 처리하는 동안 다음 accept가 지연될 수 있다.
        tokio::spawn(async move {
            if let Err(err) = handle_client(socket, peer_addr.to_string()).await {
                eprintln!("client {peer_addr} error: {err}");
            }
        });
    }
}

// 접속한 클라이언트 하나를 처리한다.
// 여기서는 가장 기본적인 echo server 패턴을 사용한다.
// 클라이언트가 보낸 데이터를 그대로 다시 돌려준다.
async fn handle_client(mut socket: TcpStream, peer: String) -> io::Result<()> {
    // 버퍼는 너무 작지도 너무 크지도 않게 적당히 둔다.
    // 실무에서는 프로토콜 성격에 따라 크기를 조정한다.
    let mut buf = [0_u8; 1024];

    loop {
        // 소켓에서 데이터를 읽는다.
        // 반환값 n은 실제로 읽은 바이트 수다.
        let n = socket.read(&mut buf).await?;

        // n == 0 이면 상대가 연결을 정상적으로 닫았다는 뜻이다.
        if n == 0 {
            println!("client disconnected: {peer}");
            break;
        }

        // 실제로 읽은 바이트만 슬라이스로 잘라서 사용해야 한다.
        // buf 전체를 쓰면 이전 데이터나 남은 빈 공간까지 섞일 수 있다.
        let received = &buf[..n];

        // 문자열로 해석 가능한지 확인한다.
        // TCP는 바이트 스트림이므로 항상 UTF-8 문자열이라고 가정하면 안 된다.
        match std::str::from_utf8(received) {
            Ok(text) => println!("received from {peer}: {text}"),
            Err(_) => println!("received non-utf8 data from {peer}: {received:?}"),
        }

        // write_all은 전달한 바이트를 전부 보낼 때까지 비동기적으로 진행한다.
        // 일반 write는 일부만 보낼 수도 있으므로 단순 예제에서는 write_all이 안전하다.
        socket.write_all(received).await?;
    }

    Ok(())
}

// 테스트용 클라이언트
// 서버에 접속해서 메시지를 보내고, echo 응답을 다시 읽는다.
async fn run_client(addr: &str, message: &str) -> io::Result<()> {
    // 서버에 TCP 연결을 생성한다.
    let mut stream = TcpStream::connect(addr).await?;
    println!("client connected to {addr}");

    // 메시지를 바이트로 보내기
    stream.write_all(message.as_bytes()).await?;
    println!("client sent: {message}");

    // 서버가 echo 해 준 응답을 읽기
    let mut buf = vec![0_u8; 1024];
    let n = stream.read(&mut buf).await?;

    let echoed = String::from_utf8_lossy(&buf[..n]);
    println!("client received: {echoed}");

    Ok(())
}
