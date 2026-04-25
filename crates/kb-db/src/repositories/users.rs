//! User repository

use chrono::Utc;
use sqlx::PgPool;

use kb_core::AppError;
use kb_core::models::{CreateUser, UpdateUser, User, UserResponse};

use crate::pool::sqlx_error_to_app_error;

/// User repository for database operations
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new user
    pub async fn create(&self, input: CreateUser, password_hash: String) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (username, email, password_hash, full_name)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(&input.username)
        .bind(&input.email)
        .bind(&password_hash)
        .bind(&input.full_name)
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?;

        Ok(user)
    }

    /// Find user by ID
    pub async fn find_by_id(&self, id: i64) -> Result<User, AppError> {
        let user =
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1 AND is_active = true")
                .bind(id)
                .fetch_optional(&self.pool)
                .await
                .map_err(sqlx_error_to_app_error)?
                .ok_or(AppError::UserNotFound)?;

        Ok(user)
    }

    /// Find user by email
    pub async fn find_by_email(&self, email: &str) -> Result<User, AppError> {
        let user =
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1 AND is_active = true")
                .bind(email)
                .fetch_optional(&self.pool)
                .await
                .map_err(sqlx_error_to_app_error)?
                .ok_or(AppError::UserNotFound)?;

        Ok(user)
    }

    /// Find user by username
    pub async fn find_by_username(&self, username: &str) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = $1 AND is_active = true",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?
        .ok_or(AppError::UserNotFound)?;

        Ok(user)
    }

    /// Update user
    pub async fn update(&self, id: i64, input: UpdateUser) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET full_name = COALESCE($2, full_name),
                avatar_url = COALESCE($3, avatar_url)
            WHERE id = $1 AND is_active = true
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&input.full_name)
        .bind(&input.avatar_url)
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_error_to_app_error)?
        .ok_or(AppError::UserNotFound)?;

        Ok(user)
    }

    /// Update last login time
    pub async fn update_last_login(&self, id: i64) -> Result<(), AppError> {
        sqlx::query("UPDATE users SET last_login_at = $1 WHERE id = $2")
            .bind(Utc::now())
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(sqlx_error_to_app_error)?;

        Ok(())
    }

    /// Check if email exists
    pub async fn email_exists(&self, email: &str) -> Result<bool, AppError> {
        let exists = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(&self.pool)
            .await
            .map_err(sqlx_error_to_app_error)?;

        Ok(exists > 0)
    }

    /// Check if username exists
    pub async fn username_exists(&self, username: &str) -> Result<bool, AppError> {
        let exists = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(&self.pool)
            .await
            .map_err(sqlx_error_to_app_error)?;

        Ok(exists > 0)
    }

    /// Convert user to response (public fields)
    pub fn to_response(user: &User) -> UserResponse {
        UserResponse {
            id: user.id,
            username: user.username.clone(),
            email: user.email.clone(),
            full_name: user.full_name.clone(),
            avatar_url: user.avatar_url.clone(),
            is_system_admin: user.is_system_admin,
        }
    }
}
