//! Ollama stub (Phase 4)

use kb_core::{AppError, CompletionRequest, CompletionResponse, StreamDelta};

pub struct OllamaClient;

impl OllamaClient {
    pub fn new(_url: &str, _model: &str) -> Result<Self, AppError> {
        Ok(Self)
    }

    pub async fn chat_completion(
        &self,
        _request: CompletionRequest,
    ) -> Result<CompletionResponse, AppError> {
        Ok(CompletionResponse {
            content: String::new(),
            model: String::new(),
            usage: None,
        })
    }

    pub async fn chat_completion_stream(
        &self,
        _request: CompletionRequest,
    ) -> Result<Vec<StreamDelta>, AppError> {
        Ok(Vec::new())
    }
}
