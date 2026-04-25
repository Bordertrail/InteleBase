//! kb-search stub - Full-text search will be implemented in Phase 3

pub mod highlight;
pub mod index;
pub mod reader;
pub mod schema;
pub mod writer;

pub use highlight::highlight;
pub use index::SearchIndex;
pub use reader::{SearchResult, search};
pub use schema::create_schema;
pub use writer::IndexWriter;
