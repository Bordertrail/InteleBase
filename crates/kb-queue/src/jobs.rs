//! Jobs stub (Phase 2)

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Job {
    ProcessDocument { document_id: uuid::Uuid },
    ReindexKB { kb_id: uuid::Uuid },
}
