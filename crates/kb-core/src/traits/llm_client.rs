//! LlmClient trait - Abstract interface for LLM providers

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::AppError;

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Completion request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub messages: Vec<ChatMessage>,
    pub model: String,
    pub stream: bool,
    pub max_tokens: Option<i32>,
    pub temperature: Option<f32>,
}

/// Completion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub content: String,
    pub model: String,
    pub usage: Option<TokenUsage>,
}

/// Token usage stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

/// Streaming delta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamDelta {
    pub token: String,
}

/// LlmClient trait - Implemented by Ollama, OpenAI, etc.
#[async_trait]
pub trait LlmClient: Send + Sync {
    /// Chat completion (non-streaming)
    async fn chat_completion(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionResponse, AppError>;

    /// Chat completion (streaming) - returns a Vec of deltas for simplicity
    async fn chat_completion_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<Vec<StreamDelta>, AppError>;
}
