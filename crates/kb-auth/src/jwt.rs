//! JWT token management

use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use kb_core::AppError;
use kb_core::config::JwtConfig;

/// JWT claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64, // User ID
    pub username: String,
    pub email: String,
    pub is_system_admin: bool,
    pub exp: i64, // Expiration timestamp
    pub iat: i64, // Issued at timestamp
    #[serde(rename = "type")]
    pub token_type: TokenType,
}

/// Token type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    Access,
    Refresh,
}

/// Token pair response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

/// Create access token
pub fn create_access_token(
    user_id: i64,
    username: &str,
    email: &str,
    is_admin: bool,
    config: &JwtConfig,
) -> Result<String, AppError> {
    let now = Utc::now();
    let exp = now + Duration::seconds(config.access_ttl);

    let claims = Claims {
        sub: user_id,
        username: username.to_string(),
        email: email.to_string(),
        is_system_admin: is_admin,
        exp: exp.timestamp(),
        iat: now.timestamp(),
        token_type: TokenType::Access,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
    .map_err(|e| AppError::JwtError(e.to_string()))
}

/// Create refresh token
pub fn create_refresh_token(
    user_id: i64,
    username: &str,
    email: &str,
    is_admin: bool,
    config: &JwtConfig,
) -> Result<String, AppError> {
    let now = Utc::now();
    let exp = now + Duration::seconds(config.refresh_ttl);

    let claims = Claims {
        sub: user_id,
        username: username.to_string(),
        email: email.to_string(),
        is_system_admin: is_admin,
        exp: exp.timestamp(),
        iat: now.timestamp(),
        token_type: TokenType::Refresh,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
    .map_err(|e| AppError::JwtError(e.to_string()))
}

/// Create both access and refresh tokens
pub fn create_token_pair(
    user_id: i64,
    username: &str,
    email: &str,
    is_admin: bool,
    config: &JwtConfig,
) -> Result<TokenPair, AppError> {
    let access_token = create_access_token(user_id, username, email, is_admin, config)?;
    let refresh_token = create_refresh_token(user_id, username, email, is_admin, config)?;

    Ok(TokenPair {
        access_token,
        refresh_token,
        expires_in: config.access_ttl,
    })
}

/// Validate token and extract claims
pub fn validate_token(token: &str, config: &JwtConfig) -> Result<Claims, AppError> {
    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| AppError::JwtError(e.to_string()))?;

    Ok(decoded.claims)
}

/// Validate access token
pub fn validate_access_token(token: &str, config: &JwtConfig) -> Result<Claims, AppError> {
    let claims = validate_token(token, config)?;

    if claims.token_type != TokenType::Access {
        return Err(AppError::JwtError("Invalid token type".to_string()));
    }

    Ok(claims)
}

/// Validate refresh token
pub fn validate_refresh_token(token: &str, config: &JwtConfig) -> Result<Claims, AppError> {
    let claims = validate_token(token, config)?;

    if claims.token_type != TokenType::Refresh {
        return Err(AppError::JwtError("Invalid token type".to_string()));
    }

    Ok(claims)
}
