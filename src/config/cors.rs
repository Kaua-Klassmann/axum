use axum::http::{HeaderValue, Method};
use tower_http::cors::{Any, CorsLayer};
use once_cell::sync::Lazy;

pub static CORS: Lazy<CorsLayer> = Lazy::new(set_cors);

fn set_cors() -> CorsLayer {
    let origin: HeaderValue = dotenvy::var("CORS_ORIGIN")
        .expect("CORS_ORIGIN not found")
        .parse()
        .unwrap();

    CorsLayer::new()
        .allow_origin(origin)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
}
