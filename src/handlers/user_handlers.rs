use argon2::password_hash::{rand_core::OsRng, SaltString, PasswordHasher};
use axum::{
    extract::State, http::StatusCode, response::IntoResponse, Json
};
use sea_orm::{ActiveValue::Set, EntityTrait};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::state::AppState;
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

#[derive(Serialize)]
pub struct CreateUserErrorResponse {
    error: String
}

#[derive(Serialize)]
struct CreateUserResponse {
    id: i32
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserPayload>
) -> Result<impl IntoResponse, (StatusCode, Json<CreateUserErrorResponse>)> {

    if payload.validate().is_err() {
        return Err((StatusCode::UNPROCESSABLE_ENTITY, Json(CreateUserErrorResponse {
            error: "Schema invalid".to_string()
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
        return Err((StatusCode::BAD_REQUEST, Json(CreateUserErrorResponse {
            error: "Failed to create account".to_string()
        })))
    }

    Ok((StatusCode::OK, Json(CreateUserResponse {
        id: res.unwrap().last_insert_id
    })))
}
