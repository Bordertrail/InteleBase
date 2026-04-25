//! OpenAI stub (Phase 4)

use kb_core::{AppError, CompletionRequest, CompletionResponse};

pub struct OpenAIClient;

impl OpenAIClient {
    pub fn new(_api_key: &str) -> Result<Self, AppError> {
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
}
