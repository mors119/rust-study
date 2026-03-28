use serde::{Deserialize, Serialize};

/// users 테이블의 한 행(row)을 표현하는 구조체
#[derive(Debug, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
}

/// POST /users 요청 body
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
}
