use axum::{
    Router,
    routing::{get, post}
};

use crate::{handlers::*, state::AppState};

pub fn configure_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/path/{name}", post(path_handlers::path))
        .route("/query", post(query_handlers::query))
        .route("/json", post(json_handlers::json))
        .route("/create_user", post(user_handlers::create_user))
}
