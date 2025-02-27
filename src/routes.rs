use axum::{
    routing::{delete, get, post, put},
    Router,
};
use tower_http::services::ServeDir;

use crate::{handlers, state::AppState};

pub fn configure_routes() -> Router<AppState> {
    Router::new()
        .merge(default_routes())
        .nest("/user", user_routes())
        .nest("/post", post_routes())
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

fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(handlers::user::create_user))
        .route("/login", post(handlers::user::login))
        .route("/update/name", put(handlers::user::update_name))
}

fn post_routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(handlers::post::create_post))
        .route("/view/user", get(handlers::post::get_all_by_user))
        .route(
            "/{uuid_post}/upload_image",
            post(handlers::post::upload_image_post),
        )
        .route("/{uuid_post}/delete", delete(handlers::post::delete_post))
        .route("/{uuid_post}/view", get(handlers::post::view_post))
        .nest_service("/view", ServeDir::new("public/uploads/posts/"))
}
