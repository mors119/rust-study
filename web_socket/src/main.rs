/* 
[클라이언트]
connect_async / new WebSocket()
        ↓
[TCP 연결]
        ↓
[WebSocket handshake]
        ↓
[양방향 stream 생성]
        ↓
split()
        ↓
send / receive 동시에 실행
        ↓
Message::Text / Binary / Close
*/

mod types;
mod server;
mod client;

use server::run_server;
use client::run_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 프로그램 시작 로그
    println!("🦀 WebSocket System Starting...");
    println!("================================");

    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        // 서버 모드
        Some("server") => {
            println!("🚀 Running in SERVER mode");
            run_server().await?;
        }

        // 클라이언트 모드
        Some("client") => {
            println!("🧪 Running in CLIENT mode");
            run_client().await?;
        }

        // 데모 모드 (실무에서는 거의 없음, 학습용)
        _ => {
            println!("📌 Demo mode (server + client)");

            // 서버를 별도 task로 실행
            let server_handle = tokio::spawn(async {
                if let Err(e) = run_server().await {
                    eprintln!("❌ server error: {e}");
                }
            });

            // 서버가 먼저 떠야 client 연결 가능
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

            // 클라이언트 실행
            if let Err(e) = run_client().await {
                eprintln!("❌ client error: {e}");
            }

            // 데모 종료
            server_handle.abort();
        }
    }

    Ok(())
}