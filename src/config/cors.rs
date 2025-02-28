use std::env;

use axum::http::HeaderValue;
use reqwest::Method;
use tower_http::cors::Any;

pub struct CorsConfig {
    pub origin: HeaderValue,
    pub methods: Vec<Method>,
    pub header: Any,
}

pub fn get_cors_config() -> CorsConfig {
    let origin = env::var("CORS_ORIGIN")
        .expect("CORS_ORIGIN not found at .env file")
        .parse::<HeaderValue>()
        .unwrap();

    let methods = vec![Method::GET, Method::POST, Method::PUT, Method::DELETE];

    let header = Any;

    CorsConfig {
        origin,
        methods,
        header,
    }
}
