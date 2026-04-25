//! Chunker stub (Phase 2)

pub fn chunk(text: &str, _chunk_size: usize, _overlap: usize) -> Vec<String> {
    if text.is_empty() {
        return Vec::new();
    }
    vec![text.to_string()]
}
