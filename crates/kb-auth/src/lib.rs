//! kb-auth - Authentication and authorization layer
//!
//! JWT token management, password hashing, AuthLayer middleware, and RBAC.

pub mod claims;
pub mod jwt;
pub mod middleware;
pub mod password;
pub mod rbac;

pub use claims::ExtractedClaims;
pub use jwt::{
    Claims, TokenPair, TokenType, create_token_pair, validate_access_token, validate_refresh_token,
};
pub use middleware::{AuthUser, OptionalAuthUser};
pub use password::{hash_password, verify_password};
pub use rbac::{PermissionLevel, check_kb_permission, require_kb_permission};
