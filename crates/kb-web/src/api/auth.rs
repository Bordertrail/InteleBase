//! Authentication API client

use crate::state::{User, LoginResponse};
use serde::Serialize;
use thiserror::Error;

const API_BASE: &str = "/api/v1";

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("Unauthorized")]
    Unauthorized,
}

#[derive(Serialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
    full_name: Option<String>,
}

#[derive(Serialize)]
struct LoginRequest {
    email: String,
    password: String,
}

/// Login with email and password
#[cfg(feature = "ssr")]
pub async fn login(email: String, password: String) -> Result<LoginResponse, ApiError> {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/auth/login", API_BASE))
        .json(&LoginRequest { email, password })
        .send()
        .await?;

    if resp.status().is_success() {
        resp.json().await.map_err(|e| ApiError::Http(e.to_string()))
    } else if resp.status() == 401 {
        Err(ApiError::Unauthorized)
    } else {
        Err(ApiError::Http(resp.text().await.unwrap_or_default()))
    }
}

/// Register a new user
#[cfg(feature = "ssr")]
pub async fn register(username: String, email: String, password: String, full_name: Option<String>) -> Result<(), ApiError> {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/auth/register", API_BASE))
        .json(&RegisterRequest { username, email, password, full_name })
        .send()
        .await?;

    if resp.status().is_success() {
        Ok(())
    } else {
        Err(ApiError::Http(resp.text().await.unwrap_or_default()))
    }
}