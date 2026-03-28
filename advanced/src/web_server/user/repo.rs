use sqlx::{FromRow, SqlitePool};

use super::model::User;

/// sqlx::query_as 로 매핑하려면 FromRow 구현이 필요하다.
#[derive(Debug, FromRow)]
struct UserRow {
    id: i64,
    name: String,
}

impl From<UserRow> for User {
    fn from(value: UserRow) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

/// 사용자 1명 조회
pub async fn find_user_by_id(pool: &SqlitePool, user_id: i64) -> Result<Option<User>, sqlx::Error> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, name
        FROM users
        WHERE id = ?
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(Into::into))
}

/// 사용자 생성
pub async fn create_user(pool: &SqlitePool, name: &str) -> Result<User, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO users (name)
        VALUES (?)
        "#,
    )
    .bind(name)
    .execute(pool)
    .await?;

    let id = result.last_insert_rowid();

    let row = sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, name
        FROM users
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(row.into())
}
