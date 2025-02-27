use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use entity::{post, user};
use sea_orm::{
    ActiveValue::Set, ColumnTrait, Condition, EntityTrait, FromQueryResult, QueryFilter,
    QuerySelect,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::{
    fs::{remove_file, File},
    io::AsyncWriteExt,
};
use uuid::Uuid;
use validator::Validate;

use crate::{config::static_server::get_static_server, jwt::JwtClaims, state::AppState};

#[derive(Deserialize, Validate)]
pub struct CreatePostPayload {
    #[validate(length(min = 1))]
    title: String,
}

pub async fn create_post(
    State(state): State<AppState>,
    token: JwtClaims,
    Json(payload): Json<CreatePostPayload>,
) -> impl IntoResponse {
    if payload.validate().is_err() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "error": "Schema invalid"
            })),
        );
    }

    let db = &state.db_conn;

    let post = post::ActiveModel {
        uuid: Set(Uuid::new_v4()),
        title: Set(payload.title),
        id_user: Set(token.user_id),
        ..Default::default()
    };

    let post_result = post::Entity::insert(post).exec(db).await;

    if post_result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to create post"
            })),
        );
    }

    let post = post_result.unwrap();

    (
        StatusCode::OK,
        Json(json!({
            "uuid_post": post.last_insert_id
        })),
    )
}

pub async fn upload_image_post(
    State(state): State<AppState>,
    token: JwtClaims,
    Path(uuid_post): Path<Uuid>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let db = &state.db_conn;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap();

        if field_name == "image" {
            let data = field.bytes().await;

            if data.is_err() {
                return (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({
                        "error": "Failed to convert image"
                    })),
                );
            }

            let post_result = post::Entity::find_by_id(uuid_post).one(db).await.unwrap();

            if post_result.is_none() {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error": "Failed to find post"
                    })),
                );
            }

            let post = post_result.unwrap();

            if post.id_user != token.user_id {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error": "Post is not your"
                    })),
                );
            }

            if post.has_image == true {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error": "Post has image"
                    })),
                );
            }

            let mut file = File::create(format!("./public/uploads/posts/{}.webp", uuid_post))
                .await
                .unwrap();
            file.write(&data.unwrap()).await.unwrap();

            let post_update = post::ActiveModel {
                uuid: Set(post.uuid),
                has_image: Set(true),
                ..Default::default()
            };

            post::Entity::update(post_update).exec(db).await.unwrap();

            return (StatusCode::OK, Json(json!({})));
        }
    }

    return (
        StatusCode::BAD_REQUEST,
        Json(json!({
            "error": "Image not found"
        })),
    );
}

#[derive(Deserialize, Validate)]
pub struct GetAllByUserPayload {
    #[validate(range(min = 1))]
    id_user: u32,
}

#[derive(FromQueryResult, Serialize)]
struct PostWithoutIdUser {
    uuid: Uuid,
    title: String,
    image: Option<String>,
}

pub async fn get_all_by_user(
    State(state): State<AppState>,
    _: JwtClaims,
    Json(payload): Json<GetAllByUserPayload>,
) -> impl IntoResponse {
    if payload.validate().is_err() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "error": "Schema invalid"
            })),
        );
    }

    let db = &state.db_conn;

    let user = user::Entity::find_by_id(payload.id_user)
        .one(db)
        .await
        .unwrap();

    if user.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "User not found"
            })),
        );
    }

    let posts = post::Entity::find()
        .filter(
            Condition::all()
                .add(post::Column::IdUser.eq(payload.id_user))
                .add(post::Column::HasImage.eq(true)),
        )
        .select_only()
        .columns([post::Column::Uuid, post::Column::Title])
        .into_model::<PostWithoutIdUser>()
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|mut post| {
            post.image = Some(format!(
                "{}/post/view/{}.webp",
                get_static_server().url,
                post.uuid
            ));
            post
        })
        .collect::<Vec<PostWithoutIdUser>>();

    (
        StatusCode::OK,
        Json(json!({
            "posts": posts
        })),
    )
}

pub async fn delete_post(
    State(state): State<AppState>,
    token: JwtClaims,
    Path(uuid_post): Path<Uuid>,
) -> impl IntoResponse {
    let db = &state.db_conn;

    let post_result = post::Entity::find_by_id(uuid_post).one(db).await.unwrap();

    if post_result.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Failed to find post"
            })),
        );
    }

    let post = post_result.unwrap();

    if post.id_user != token.user_id {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": "Post is from other user"
            })),
        );
    }

    let delete_result = post::Entity::delete_by_id(post.uuid).exec(db).await;

    if delete_result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to delete post"
            })),
        );
    }

    if post.has_image {
        remove_file(format!("./public/uploads/posts/{}.webp", uuid_post))
            .await
            .unwrap();
    }

    return (StatusCode::OK, Json(json!({})));
}

pub async fn view_post(
    State(state): State<AppState>,
    _: JwtClaims,
    Path(uuid_post): Path<Uuid>,
) -> impl IntoResponse {
    let db = &state.db_conn;

    let post_result = post::Entity::find()
        .find_also_related(user::Entity)
        .filter(
            Condition::all()
                .add(post::Column::Uuid.eq(uuid_post))
                .add(post::Column::HasImage.eq(true)),
        )
        .one(db)
        .await
        .unwrap();

    if post_result.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Failed to find post"
            })),
        );
    }

    let (post, user_result) = post_result.unwrap();

    let user = user_result.unwrap();

    (
        StatusCode::OK,
        Json(json!({
            "name": post.title,
            "image": format!("{}/post/view/{}.webp", get_static_server().url, post.uuid),
            "user": {
                "id": user.id,
                "name": user.name
            }
        })),
    )
}
