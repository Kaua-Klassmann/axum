use axum::{
    routing::{get, post},
    Router,
};
use post::get_post_routes;
use user::get_user_routes;

use crate::{handlers, state::AppState};

mod post;
mod user;

pub fn configure_routes() -> Router<AppState> {
    Router::new()
        .merge(default_routes())
        .nest("/user", get_user_routes())
        .nest("/post", get_post_routes())
        .route("/gemini", post(handlers::gemini::chat))
}

fn default_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/path/{name}", post(handlers::path::path))
        .route("/query", post(handlers::query::query))
        .route("/json", post(handlers::json::json))
        .route("/token", get(handlers::token::token))
}
