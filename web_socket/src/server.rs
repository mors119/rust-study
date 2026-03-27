// TCP → WebSocket upgrade → Stream → split → send/recv

use crate::types::ChatMessage;
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};

/// WebSocket 서버 실행
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080";

    // TCP 서버 생성
    let listener = TcpListener::bind(addr).await?;
    println!("🚀 Server listening on {addr}");

    loop {
        // 1️⃣ TCP 연결 수락 (blocking 아님, async 대기)
        let (stream, addr) = listener.accept().await?;
        println!("🔗 New TCP connection: {addr}");

        // 2️⃣ 각 클라이언트를 독립 task로 처리 (핵심)
        tokio::spawn(handle_connection(stream, addr));
    }
}

/// 클라이언트 연결 처리 (핵심 로직)
async fn handle_connection(stream: TcpStream, addr: std::net::SocketAddr) {
    // 3️⃣ WebSocket handshake (HTTP → WS upgrade)
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("❌ handshake failed: {addr} - {e}");
            return;
        }
    };

    println!("✅ WebSocket established: {addr}");

    // 4️⃣ stream 분리 (send / receive)
    let (mut sender, mut receiver) = ws_stream.split();

    // 5️⃣ 초기 메시지
    let welcome = ChatMessage::new(
        "server".into(),
        format!("{addr} connected"),
    );

    if let Ok(json) = serde_json::to_string(&welcome) {
        sender.send(Message::Text(json.into())).await.ok();
    }

    // 6️⃣ 메시지 처리 루프 (핵심)
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("📨 {addr}: {text}");

                // JSON 파싱 시도
                if let Ok(chat) = serde_json::from_str::<ChatMessage>(&text) {
                    let response = ChatMessage::new(
                        "echo".into(),
                        chat.content,
                    );

                    let json = serde_json::to_string(&response).unwrap();
                    sender.send(Message::Text(json.into())).await.ok();
                }
            }

            Ok(Message::Binary(data)) => {
                println!("📁 binary {} bytes", data.len());
                sender.send(Message::Binary(data)).await.ok();
            }

            Ok(Message::Close(_)) => {
                println!("👋 {addr} disconnected");
                break;
            }

            Err(e) => {
                eprintln!("❌ error {addr}: {e}");
                break;
            }

            _ => {}
        }
    }
}