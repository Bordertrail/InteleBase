//! Repositories module - Database CRUD operations

pub mod audit_log;
pub mod chunks;
pub mod documents;
pub mod knowledge_bases;
pub mod permissions;
pub mod roles;
pub mod users;

pub use audit_log::AuditLogRepository;
pub use chunks::ChunkRepository;
pub use documents::DocumentRepository;
pub use knowledge_bases::KnowledgeBaseRepository;
pub use permissions::PermissionRepository;
pub use roles::RoleRepository;
pub use users::UserRepository;
