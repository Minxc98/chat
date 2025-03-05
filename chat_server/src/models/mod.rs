mod user;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug,Clone,Serialize,FromRow, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
}

