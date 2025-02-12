use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use entity::post;
use sea_orm::{EntityTrait, Set};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{jwt::JwtClaims, state::AppState};

#[derive(Deserialize, Validate)]
pub struct CreatePostPayload {
    #[validate(length(min = 1))]
    title: String,
    #[validate(length(min = 1))]
    image: String
}

pub async fn create_post(
    State(state): State<AppState>,
    token: JwtClaims,
    Json(payload): Json<CreatePostPayload>
) -> impl IntoResponse {

    if payload.validate().is_err() {
        return (StatusCode::UNPROCESSABLE_ENTITY, Json(json!({
            "error": "Schema invalid"
        })))
    }

    let db = &state.db_conn;

    let post = post::ActiveModel {
        uuid: Set(Uuid::new_v4().into()),
        title: Set(payload.title),
        image: Set(payload.image),
        id_user: Set(token.user_id)
    };

    let post_result = post::Entity::insert(post).exec(db).await;

    if post_result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
            "error": "Failed to create post"
        })))
    }

    (StatusCode::OK, Json(json!({})))
}
