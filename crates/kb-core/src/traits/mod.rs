//! Traits module - Abstract interfaces for external services

pub mod llm_client;
pub mod search_engine;
pub mod storage;
pub mod vector_store;

pub use llm_client::{
    ChatMessage, CompletionRequest, CompletionResponse, LlmClient, StreamDelta, TokenUsage,
};
pub use search_engine::{IndexableDocument, ScoredDoc, SearchEngine, SearchFilters};
pub use storage::ObjectStorage;
pub use vector_store::{ScoredPoint, VectorPoint, VectorSearchFilters, VectorStore};
