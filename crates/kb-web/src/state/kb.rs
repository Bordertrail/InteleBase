//! Knowledge base state

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Knowledge base data from API
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct KnowledgeBase {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: i64,
    pub created_at: String,
}

/// Paginated result from API
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i64,
}

/// Member in a knowledge base
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct KbMember {
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub role_name: String,
}

/// Global knowledge base state
#[derive(Clone, Default)]
pub struct KbState {
    pub list: RwSignal<Vec<KnowledgeBase>>,
    pub current_page: RwSignal<i32>,
    pub total_pages: RwSignal<i64>,
    pub current_kb: RwSignal<Option<KnowledgeBase>>,
    pub members: RwSignal<Vec<KbMember>>,
}