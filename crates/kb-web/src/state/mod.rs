//! Application state management

pub mod auth;
pub mod kb;

pub use auth::{AuthState, User, LoginResponse};
pub use kb::{KbState, KnowledgeBase, KbMember, PaginatedResult};