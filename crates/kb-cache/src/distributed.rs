//! Distributed cache stub - Redis (Phase 2)

use kb_core::AppError;

pub struct DistributedCache;

impl DistributedCache {
    pub fn new(url: &str) -> Result<Self, AppError> {
        Ok(Self)
    }
}
