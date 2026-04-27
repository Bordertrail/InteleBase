//! API client functions

pub mod auth;
pub mod kb;

pub use auth::{login, register, ApiError};
pub use kb::{list_kbs, get_kb, create_kb, delete_kb, list_members, add_member, remove_member};