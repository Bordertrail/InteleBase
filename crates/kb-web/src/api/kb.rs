//! Knowledge base API client

use crate::state::{KbMember, KnowledgeBase, PaginatedResult};
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
struct CreateKbRequest {
    name: String,
    description: Option<String>,
}

#[derive(Serialize)]
struct AddMemberRequest {
    user_id: i64,
    role: String,
}

/// List knowledge bases
#[cfg(feature = "ssr")]
pub async fn list_kbs(page: i32, per_page: i32, token: &str) -> Result<PaginatedResult<KnowledgeBase>, ApiError> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/knowledge-bases?page={}&per_page={}", API_BASE, page, per_page))
        .bearer_auth(token)
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

/// Get a knowledge base by ID
#[cfg(feature = "ssr")]
pub async fn get_kb(id: i64, token: &str) -> Result<KnowledgeBase, ApiError> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/knowledge-bases/{}", API_BASE, id))
        .bearer_auth(token)
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

/// Create a new knowledge base
#[cfg(feature = "ssr")]
pub async fn create_kb(name: String, description: Option<String>, token: &str) -> Result<KnowledgeBase, ApiError> {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/knowledge-bases", API_BASE))
        .bearer_auth(token)
        .json(&CreateKbRequest { name, description })
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

/// Delete a knowledge base
#[cfg(feature = "ssr")]
pub async fn delete_kb(id: i64, token: &str) -> Result<(), ApiError> {
    let client = reqwest::Client::new();
    let resp = client
        .delete(format!("{}/knowledge-bases/{}", API_BASE, id))
        .bearer_auth(token)
        .send()
        .await?;

    if resp.status().is_success() {
        Ok(())
    } else if resp.status() == 401 {
        Err(ApiError::Unauthorized)
    } else {
        Err(ApiError::Http(resp.text().await.unwrap_or_default()))
    }
}

/// List members of a knowledge base
#[cfg(feature = "ssr")]
pub async fn list_members(kb_id: i64, token: &str) -> Result<Vec<KbMember>, ApiError> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/knowledge-bases/{}/members", API_BASE, kb_id))
        .bearer_auth(token)
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

/// Add a member to a knowledge base
#[cfg(feature = "ssr")]
pub async fn add_member(kb_id: i64, user_id: i64, role: String, token: &str) -> Result<KbMember, ApiError> {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/knowledge-bases/{}/members", API_BASE, kb_id))
        .bearer_auth(token)
        .json(&AddMemberRequest { user_id, role })
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

/// Remove a member from a knowledge base
#[cfg(feature = "ssr")]
pub async fn remove_member(kb_id: i64, user_id: i64, token: &str) -> Result<(), ApiError> {
    let client = reqwest::Client::new();
    let resp = client
        .delete(format!("{}/knowledge-bases/{}/members/{}", API_BASE, kb_id, user_id))
        .bearer_auth(token)
        .send()
        .await?;

    if resp.status().is_success() {
        Ok(())
    } else if resp.status() == 401 {
        Err(ApiError::Unauthorized)
    } else {
        Err(ApiError::Http(resp.text().await.unwrap_or_default()))
    }
}