//! kb-db - Database layer with PostgreSQL connection and repositories
//!
//! Provides PgPool management and CRUD operations for all entities.

pub mod pagination;
pub mod pool;
pub mod repositories;

pub use pagination::{PaginatedResult, PaginationQuery};
pub use pool::{PgPool, create_pool, sqlx_error_to_app_error};
pub use repositories::*;

// Re-export sqlx for convenience
pub use sqlx;
