use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    PasswordHash,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{jwt::JwtClaims, services::email::send_email, state::AppState};
use entity::user;

#[derive(Deserialize, Validate)]
pub struct CreateUserPayload {
    #[validate(length(min = 1))]
    name: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserPayload>,
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
    let argon2 = &state.argon2;

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let user_res = user::Entity::find()
        .filter(user::Column::Email.eq(payload.email.clone()))
        .one(db)
        .await
        .unwrap();

    if user_res.is_some() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "User found on database"
            })),
        );
    }

    let validation = Uuid::new_v4();

    let user = user::ActiveModel {
        name: Set(payload.name),
        email: Set(payload.email.clone()),
        password: Set(password_hash),
        activation: Set(Some(validation)),
        ..Default::default()
    };

    let email_res = send_email(
        payload.email,
        "Active account".to_string(),
        format!("http://localhost:3000/user/activate/{}", validation),
    )
    .await;

    if email_res.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to send email"
            })),
        );
    }

    let res = user::Entity::insert(user).exec(db).await;

    if res.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to create account"
            })),
        );
    }

    (
        StatusCode::OK,
        Json(json!({
            "id": res.unwrap().last_insert_id
        })),
    )
}

pub async fn active_user(
    State(state): State<AppState>,
    Path(validation_uuid): Path<Uuid>,
) -> impl IntoResponse {
    let db = &state.db_conn;

    let res = user::Entity::find()
        .filter(user::Column::Activation.eq(validation_uuid))
        .one(db)
        .await
        .unwrap();

    if res.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Invalid activation link"
            })),
        );
    }

    let mut user = res.unwrap().into_active_model();
    user.activation = Set(None);

    let res = user.save(db).await;

    if res.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to activate account"
            })),
        );
    }

    (StatusCode::OK, Json(json!({})))
}

#[derive(Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
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
    let argon2 = &state.argon2;

    let user_result = user::Entity::find()
        .filter(user::Column::Email.eq(payload.email))
        .one(db)
        .await
        .unwrap();

    if user_result.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "User not found"
            })),
        );
    }

    let user_data = user_result.unwrap();

    if user_data.activation.is_some() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "User not activated"
            })),
        );
    }

    let parsed_hash = PasswordHash::new(&user_data.password).unwrap();

    if argon2
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Password incorrect"
            })),
        );
    }

    let jwt_token = JwtClaims::new(user_data.id).gen_token();

    (
        StatusCode::OK,
        Json(json!({
            "token": jwt_token
        })),
    )
}

#[derive(Deserialize, Validate)]
pub struct UpdateUserPayload {
    #[validate(length(min = 1))]
    name: String,
}

pub async fn update_name(
    State(state): State<AppState>,
    token: JwtClaims,
    Json(payload): Json<UpdateUserPayload>,
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

    let user = user::ActiveModel {
        id: Set(token.user_id),
        name: Set(payload.name),
        ..Default::default()
    };

    let update_res = user::Entity::update(user)
        .filter(user::Column::Id.eq(token.user_id))
        .exec(db)
        .await;

    if update_res.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to update user"
            })),
        );
    }

    (StatusCode::OK, Json(json!({})))
}
