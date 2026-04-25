//! kb-llm stub - LLM integration will be implemented in Phase 4

pub mod ollama;
pub mod openai;
pub mod types;

pub use ollama::OllamaClient;
pub use openai::OpenAIClient;
