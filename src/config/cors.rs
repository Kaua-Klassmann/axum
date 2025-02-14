use std::env;

use axum::http::{HeaderValue, Method};
use tower_http::cors::{Any, CorsLayer};

pub fn get_cors() -> CorsLayer {
    let origin = env::var("CORS_ORIGIN")
        .expect("CORS_ORIGIN not found at .env file")
        .parse::<HeaderValue>()
        .unwrap();

    CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
}
