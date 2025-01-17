use std::sync::LazyLock;

use tower_http::cors::{Any, CorsLayer};

pub static CORS: LazyLock<CorsLayer> = LazyLock::new(|| {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
});