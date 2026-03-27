// connect → split → send task / receive task
use crate::types::ChatMessage;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
    let url = "ws://127.0.0.1:8080";

    println!("🔌 connecting to {url}");

    // 1️⃣ WebSocket 연결
    let (ws_stream, _) = connect_async(url).await?;
    println!("✅ connected");

    // 2️⃣ split (핵심)
    let (mut sender, mut receiver) = ws_stream.split();

    // 3️⃣ 수신 task (비동기 분리)
    let recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    println!("📨 recv: {text}");
                }
                Ok(Message::Close(_)) => break,
                Err(e) => {
                    eprintln!("❌ recv error: {e}");
                    break;
                }
                _ => {}
            }
        }
    });

    // 4️⃣ 메시지 전송
    let msg = ChatMessage::new("client".into(), "hello".into());
    let json = serde_json::to_string(&msg)?;

    sender.send(Message::Text(json.into())).await?;

    // 종료
    sender.send(Message::Close(None)).await?;

    recv_task.await?;

    Ok(())
}