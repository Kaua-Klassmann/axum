use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use entity::{like, post};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use serde_json::json;
use uuid::Uuid;

use crate::{jwt::JwtClaims, state::AppState};

pub async fn like_post(
    State(state): State<AppState>,
    token: JwtClaims,
    Path(post_uuid): Path<Uuid>,
) -> impl IntoResponse {
    let db = &state.db_conn;

    let post_result = post::Entity::find_by_id(post_uuid).one(db).await.unwrap();

    if post_result.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Post not found"
            })),
        );
    }

    let like = like::ActiveModel {
        id_post: Set(post_uuid),
        id_user: Set(token.user_id),
        ..Default::default()
    };

    let like_result = like::Entity::insert(like).exec(db).await;

    if like_result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to like post"
            })),
        );
    }

    (StatusCode::OK, Json(json!({})))
}

pub async fn view_all_likes(State(state): State<AppState>, token: JwtClaims) -> impl IntoResponse {
    let db = &state.db_conn;

    let likes_result = like::Entity::find()
        .find_also_related(post::Entity)
        .filter(like::Column::IdUser.eq(token.user_id))
        .all(db)
        .await;

    if likes_result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to find posts"
            })),
        );
    }

    let likes = likes_result.unwrap();

    let posts: Vec<_> = likes.into_iter().map(|(_, post)| post).collect();

    (
        StatusCode::OK,
        Json(json!({
            "posts": posts
        })),
    )
}
