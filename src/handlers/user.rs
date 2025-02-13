use argon2::{password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString}, PasswordHash};
use axum::{
    extract::State, http::StatusCode, response::IntoResponse, Json
};
use sea_orm::{ActiveValue::Set, EntityTrait, QueryFilter, ColumnTrait};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::{jwt::JwtClaims, state::AppState};
use entity::user;

#[derive(Deserialize, Validate)]
pub struct CreateUserPayload {
    #[validate(length(min = 1))]
    name: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserPayload>
) -> impl IntoResponse {

    if payload.validate().is_err() {
        return (StatusCode::UNPROCESSABLE_ENTITY, Json(json!({
            "error": "Schema invalid"
        })))
    }

    let db = &state.db_conn;
    let argon2 = &state.argon2;

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2.hash_password(payload.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let user = user::ActiveModel {
        name: Set(payload.name),
        email: Set(payload.email),
        password: Set(password_hash),
        ..Default::default()
    };

    let res = user::Entity::insert(user).exec(db).await;

    if res.is_err() {
        return (StatusCode::BAD_REQUEST, Json(json!({
            "error": "Failed to create account"
        })))
    }

    (StatusCode::OK, Json(json!({
        "id": res.unwrap().last_insert_id
    })))
}

#[derive(Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>
) -> impl IntoResponse {

    if payload.validate().is_err() {
        return (StatusCode::UNPROCESSABLE_ENTITY, Json(json!({
            "error": "Schema invalid"
        })))
    }

    let db = &state.db_conn;
    let argon2 = &state.argon2;

    let user_result = user::Entity::find()
        .filter(user::Column::Email.eq(payload.email))
        .one(db)
        .await
        .unwrap();

    if user_result.is_none() {
        return (StatusCode::BAD_REQUEST, Json(json!({
            "error": "User not found"
        })))
    }

    let user_data = user_result.unwrap();
    let parsed_hash = PasswordHash::new(&user_data.password).unwrap();

    if argon2.verify_password(
        payload.password.as_bytes(),
        &parsed_hash
    ).is_err() {
        return (StatusCode::BAD_REQUEST, Json(json!({
            "error": "Password incorrect"
        })))
    }

    let jwt_token = JwtClaims::new(user_data.id).gen_token();

    (StatusCode::OK, Json(json!({
        "token": jwt_token
    })))
}

#[derive(Deserialize, Validate)]
pub struct UpdateUserPayload {
    #[validate(length(min = 1))]
    name: String
}

pub async fn update_name(
    State(state): State<AppState>,
    token: JwtClaims,
    Json(payload): Json<UpdateUserPayload>
) -> impl IntoResponse {

    if payload.validate().is_err() {
        return (StatusCode::UNPROCESSABLE_ENTITY, Json(json!({
            "error": "Schema invalid"
        })))
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
        return (StatusCode::BAD_REQUEST, Json(json!({
            "error": "Failed to update user"
        })))
    }

    (StatusCode::OK, Json(json!({})))
}