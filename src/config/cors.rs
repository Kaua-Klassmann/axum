use std::{env, sync::LazyLock};

use axum::http::HeaderValue;
use tower_http::cors::{Any, CorsLayer};

pub static CORS: LazyLock<CorsLayer> = LazyLock::new(configure_cors);

fn configure_cors() -> CorsLayer {
    let origin = env::var("CORS_ORIGIN")
        .expect("CORS_ORIGIN not found at .env file")
        .parse::<HeaderValue>()
        .unwrap();

    CorsLayer::new()
        .allow_origin(origin)
        .allow_methods(Any)
        .allow_headers(Any)
}