// 한번에 router handler까지 구현
// https://github.com/joelparkerhenderson/demo-rust-axum 가 제일 보기 좋음.

/*
POST 동작 확인
```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name":"mors"}'
```

GET 동작 확인
```bash
curl http://localhost:3000/users/1
```
*/

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::SqlitePool;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::web_server::db;
use crate::web_server::user;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tracing 로그 초기화 (main에서 할 때 사용)
    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::EnvFilter::new("info,tower_http=info"))
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    // SQLite 파일 DB
    let pool: SqlitePool = db::init_db().await?;
    db::init_schema(&pool).await?;

    let app = Router::new()
        .route("/users/{id}", get(user::handler::get_user))
        .route("/users", post(user::handler::create_user))
        .with_state(pool);
    // tower-http 미들웨어 예시
    // .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("localhost:3000").await?;

    println!("server running on http://localhost:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
