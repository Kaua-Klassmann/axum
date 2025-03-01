use axum::{
    routing::{delete, get, post},
    Router,
};
use tower_http::services::ServeDir;

use crate::{handlers, state::AppState};

pub fn get_post_routes() -> Router<AppState> {
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
