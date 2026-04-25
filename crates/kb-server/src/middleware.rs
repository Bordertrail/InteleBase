//! Server-level middleware configuration

use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    trace::TraceLayer,
};

/// Create CORS layer
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any) // TODO: Restrict in production
        .allow_methods(Any)
        .allow_headers(Any)
}

/// Create trace layer for HTTP requests
pub fn trace_layer() -> impl Clone {
    TraceLayer::new_for_http()
}

/// Create request body limit layer (50MB)
pub fn body_limit_layer(max_size_mb: usize) -> RequestBodyLimitLayer {
    RequestBodyLimitLayer::new(max_size_mb * 1024 * 1024)
}

/// Create compression layer
pub fn compression_layer() -> CompressionLayer {
    CompressionLayer::new().gzip(true)
}
