use lazy_static::lazy_static;
use tower_http::cors::{Any, CorsLayer};

lazy_static! {
    pub static ref CORS: CorsLayer = set_cors();
}

fn set_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}
