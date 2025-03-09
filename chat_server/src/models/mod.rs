pub(crate) mod user;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[sqlx(default)]
    pub password_hash: Option<String>,
    pub ws_id: i32,
    pub created_at: Option<NaiveDateTime>,
}
