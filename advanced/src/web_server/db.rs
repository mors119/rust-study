use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

/// SQLite 커넥션 풀 생성
pub async fn init_db() -> Result<SqlitePool, Box<dyn std::error::Error>> {
    // 현재 실행 디렉터리 기준으로 data 폴더를 만든다.
    let base_dir: PathBuf = std::env::current_dir()?.join("data");
    fs::create_dir_all(&base_dir)?; // 상위 디렉터리 없으면 생성

    let db_path = base_dir.join("app.db");

    // Windows까지 생각하면 URL 문자열 직접 조립보다 옵션 객체가 더 안전하다.
    let options = SqliteConnectOptions::from_str(&format!("sqlite://{}", db_path.display()))?
        .create_if_missing(true);

    println!("current_dir = {:?}", std::env::current_dir()?);
    println!("db_path = {:?}", db_path);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    Ok(pool)
}

/// 앱 시작 시 필요한 테이블 생성
pub async fn init_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
