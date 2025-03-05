use argon2::{
    password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHash, PasswordHasher, PasswordVerifier
};

use crate::{AppError, User};


impl User {
    pub async fn find_by_username(
        pool: &sqlx::PgPool,
        username: &str,
    ) -> Result<Option<Self>, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password, COALESCE(created_at, NOW())::TIMESTAMP as created_at 
            FROM users 
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(pool)
        .await?;
        Ok(user)
    }

    //create user
    pub async fn create(
        pool: &sqlx::PgPool,
        username: &str,
        password: &str,
    ) -> Result<Self, AppError> {
        let hashed_password = hash_password(password)?;
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, password) 
            VALUES ($1, $2) 
            RETURNING id, username, password, created_at
            "#,
            username,
            hashed_password
        )
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    pub async fn verify_password(
        &self,
        username: &str,
        password: &str,
        pool: &sqlx::PgPool,
    ) -> Result<bool, AppError> {
        let user = Self::find_by_username(pool, username).await?;
        if let Some(user) = user {
            verify_password(password, &user.password)
        } else {
            Err(AppError::InvalidCredentials)
        }
    }
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let key = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AppError::PasswordHashError)?;
    Ok(key.to_string())
}


fn verify_password(password: &str, hashed_password: &str) -> Result<bool, AppError> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hashed_password).map_err(|_| AppError::PasswordHashError)?;
    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}
