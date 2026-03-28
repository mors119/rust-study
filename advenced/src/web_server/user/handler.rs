use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::SqlitePool;

use super::{
    model::{CreateUserRequest, User},
    repo,
};

/// GET /users/:id
pub async fn get_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<User>, StatusCode> {
    let user = repo::find_user_by_id(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match user {
        Some(user) => Ok(Json(user)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// POST /users
pub async fn create_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let user = repo::create_user(&pool, &payload.name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(user)))
}
