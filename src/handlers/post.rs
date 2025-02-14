use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use entity::{post, user};
use sea_orm::{
    ActiveValue::Set,
    ColumnTrait,
    EntityTrait,
    FromQueryResult,
    QueryFilter,
    QuerySelect
};
use serde::{Deserialize, Serialize};
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
        uuid: Set(Uuid::new_v4()),
        title: Set(payload.title),
        image: Set(payload.image),
        id_user: Set(token.user_id)
    };

    let post_result = post::Entity::insert(post)
        .exec(db)
        .await;

    if post_result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
            "error": "Failed to create post"
        })))
    }

    (StatusCode::OK, Json(json!({})))
}

#[derive(Deserialize, Validate)]
pub struct GetAllByUserPayload {
    #[validate(range(min = 1))]
    id_user: i32
}

#[derive(FromQueryResult, Serialize)]
struct PostWithoutIdUser {
    uuid: Uuid,
    title: String,
    image: String
}

pub async fn get_all_by_user(
    State(state): State<AppState>,
    _: JwtClaims,
    Json(payload): Json<GetAllByUserPayload>
) -> impl IntoResponse {

    if payload.validate().is_err() {
        return (StatusCode::UNPROCESSABLE_ENTITY, Json(json!({
            "error": "Schema invalid"
        })))
    }

    let db = &state.db_conn;

    let user = user::Entity::find_by_id(payload.id_user)
        .one(db)
        .await
        .unwrap();

    if user.is_none() {
        return (StatusCode::BAD_REQUEST, Json(json!({
            "error": "User not found"
        })))
    }

    let posts = post::Entity::find()
        .filter(post::Column::IdUser.eq(payload.id_user))
        .select_only()
        .columns([
            post::Column::Uuid,
            post::Column::Title,
            post::Column::Image
        ])
        .into_model::<PostWithoutIdUser>()
        .all(db)
        .await
        .unwrap();

    (StatusCode::OK, Json(json!({
        "posts": posts
    })))
}

pub async fn delete_post(
    State(state): State<AppState>,
    token: JwtClaims,
    Path(uuid_post): Path<Uuid>
) -> impl IntoResponse {
    
    let db = &state.db_conn;

    let post_result = post::Entity::find_by_id(uuid_post)
        .one(db)
        .await
        .unwrap();

    if post_result.is_none() {
        return (StatusCode::BAD_REQUEST, Json(json!({
            "error": "Failed to find post"
        })))
    }

    let post = post_result.unwrap();

    if post.id_user != token.user_id {
        return (StatusCode::UNAUTHORIZED, Json(json!({
            "error": "Post is from other user"
        })))
    }

    let delete_result = post::Entity::delete_by_id(post.uuid)
        .exec(db)
        .await;

    if delete_result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
            "error": "Failed to delete post"
        })))
    }

    (StatusCode::OK, Json(json!({})))
}

pub async fn view_post(
    State(state): State<AppState>,
    _: JwtClaims,
    Path(uuid_post): Path<Uuid>
) -> impl IntoResponse {

    let db = &state.db_conn;

    let post_result = post::Entity::find()
        .find_also_related(user::Entity)
        .filter(post::Column::Uuid.eq(uuid_post))
        .one(db)
        .await
        .unwrap();

    if post_result.is_none() {
        return (StatusCode::BAD_REQUEST, Json(json!({
            "error": "Failed to find post"
        })))
    }

    let (post, user_result) = post_result.unwrap();

    let user = user_result.unwrap();

    (StatusCode::OK, Json(json!({
        "uuid": post.uuid,
        "name": post.title,
        "image": post.image,
        "user": {
            "id": user.id,
            "name": user.name
        }
    })))
}