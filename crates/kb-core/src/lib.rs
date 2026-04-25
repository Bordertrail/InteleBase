//! kb-core - Core models, traits, error types, and configuration
//!
//! This crate contains the foundational types shared across all other kb-* crates.

pub mod config;
pub mod error;
pub mod models;
pub mod traits;

pub use config::AppConfig;
pub use error::{AppError, ErrorDetail, ErrorResponse};
pub use models::*;
pub use traits::*;
