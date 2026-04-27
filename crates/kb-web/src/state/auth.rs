//! Authentication state

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// User data from API
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
    pub is_system_admin: bool,
}

/// Login response from API
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub user: User,
}

/// Global authentication state
#[derive(Clone, Default)]
pub struct AuthState {
    pub token: RwSignal<Option<String>>,
    pub user: RwSignal<Option<User>>,
}

impl AuthState {
    pub fn is_authenticated(&self) -> bool {
        self.token.read().is_some()
    }

    pub fn clear(&self) {
        self.token.set(None);
        self.user.set(None);
    }
}