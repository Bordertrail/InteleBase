//! Authentication routes

use crate::error::ServerError;
use crate::state::AppState;
use axum::{
    Extension, Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use kb_auth::{TokenPair, create_token_pair, hash_password, verify_password};
use kb_core::AppError;
use kb_core::config::JwtConfig;
use kb_core::models::{CreateUser, LoginRequest, UserResponse};
use kb_db::UserRepository;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Auth routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh_token))
        .route("/logout", post(logout))
        .route("/me", get(me))
}

/// Register request
#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
}

/// Register response
#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub user_id: i64,
    pub username: String,
    pub email: String,
}

/// Login response
#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub user: UserResponse,
}

/// Refresh request
#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

/// Logout request
#[derive(Debug, Deserialize, ToSchema)]
pub struct LogoutRequest {
    pub refresh_token: String,
}

/// Register a new user account
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = RegisterResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Duplicate email or username"),
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Extension(config): Extension<JwtConfig>,
    Json(req): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<RegisterResponse>), ServerError> {
    // Validate input
    if req.username.len() < 3 || req.username.len() > 64 {
        return Err(
            AppError::ValidationError("Username must be 3-64 characters".to_string()).into(),
        );
    }
    if req.password.len() < 8 {
        return Err(AppError::ValidationError(
            "Password must be at least 8 characters".to_string(),
        )
        .into());
    }

    // Check duplicates
    let user_repo = UserRepository::new(state.db.clone());
    if user_repo.email_exists(&req.email).await? {
        return Err(AppError::DuplicateEntry("Email already registered".to_string()).into());
    }
    if user_repo.username_exists(&req.username).await? {
        return Err(AppError::DuplicateEntry("Username already taken".to_string()).into());
    }

    // Hash password
    let password_hash = hash_password(&req.password)?;

    // Create user
    let create_input = CreateUser {
        username: req.username,
        email: req.email,
        password: req.password,
        full_name: req.full_name,
    };

    let user = user_repo.create(create_input, password_hash).await?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterResponse {
            user_id: user.id,
            username: user.username,
            email: user.email,
        }),
    ))
}

/// Login with email and password
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials"),
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Extension(config): Extension<JwtConfig>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ServerError> {
    // Find user
    let user_repo = UserRepository::new(state.db.clone());
    let user = user_repo.find_by_email(&req.email).await?;

    // Verify password
    if !verify_password(&req.password, &user.password_hash)? {
        return Err(AppError::InvalidCredentials.into());
    }

    // Update last login
    user_repo.update_last_login(user.id).await?;

    // Create tokens
    let tokens = create_token_pair(
        user.id,
        &user.username,
        &user.email,
        user.is_system_admin,
        &config,
    )?;

    Ok(Json(LoginResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        expires_in: tokens.expires_in,
        user: UserRepository::to_response(&user),
    }))
}

/// Refresh access token using refresh token
#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    tag = "auth",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = TokenPair),
        (status = 401, description = "Invalid or expired refresh token"),
    )
)]
pub async fn refresh_token(
    Extension(config): Extension<JwtConfig>,
    Json(req): Json<RefreshRequest>,
) -> Result<Json<TokenPair>, ServerError> {
    // Validate refresh token
    let claims = kb_auth::validate_refresh_token(&req.refresh_token, &config)?;

    // Create new tokens
    let tokens = create_token_pair(
        claims.sub,
        &claims.username,
        &claims.email,
        claims.is_system_admin,
        &config,
    )?;

    Ok(Json(tokens))
}

/// Logout and invalidate refresh token
#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    tag = "auth",
    request_body = LogoutRequest,
    responses(
        (status = 204, description = "Logged out successfully"),
    )
)]
pub async fn logout(Json(_req): Json<LogoutRequest>) -> StatusCode {
    // TODO: Add token to Redis blacklist for revocation
    StatusCode::NO_CONTENT
}

/// Get current authenticated user
#[utoipa::path(
    get,
    path = "/api/v1/auth/me",
    tag = "auth",
    responses(
        (status = 200, description = "Current user info", body = UserResponse),
        (status = 401, description = "Not authenticated"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn me(
    State(state): State<AppState>,
    kb_auth::AuthUser(claims): kb_auth::AuthUser,
) -> Result<Json<UserResponse>, ServerError> {
    let user_repo = UserRepository::new(state.db.clone());
    let user = user_repo.find_by_id(claims.user_id).await?;

    Ok(Json(UserRepository::to_response(&user)))
}
