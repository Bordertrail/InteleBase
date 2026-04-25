//! kb-worker stub - Background processing will be implemented in Phase 2

mod processor;

use kb_core::config::AppConfig;

fn main() -> Result<(), kb_core::AppError> {
    let config = AppConfig::from_env()?;
    run(config)
}

pub fn run(_config: AppConfig) -> Result<(), kb_core::AppError> {
    Ok(())
}
