//! Prompt templates stub (Phase 4)

pub fn build_rag_prompt(context: &str, question: &str) -> String {
    format!(
        "System: You are a helpful assistant. Answer based ONLY on the provided context.\n\nContext: {}\n\nQuestion: {}\n\nAnswer:",
        context, question
    )
}
