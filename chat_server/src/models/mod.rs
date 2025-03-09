pub(crate) mod user;
pub(crate) mod workspace;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[sqlx(default)]
    pub password_hash: Option<String>,
    pub ws_id: i32,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, FromRow, Deserialize)]
pub struct Workspace {
    pub id: i32,
    pub name: String,
    pub owner_id: i32,
    pub created_at: Option<NaiveDateTime>,
}
