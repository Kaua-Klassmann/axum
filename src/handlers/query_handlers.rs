use std::collections::HashMap;

use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Payload {
    name: String
}

pub async fn query(
    Query(payload): Query<Payload>
) -> impl IntoResponse {
    (StatusCode::OK, format!("Hello {}", payload.name))
}

pub async fn generic_query(
    Query(payload): Query<HashMap<String, String>>
) -> impl IntoResponse {
    match payload.get("name") {
        Some(name) => (StatusCode::OK, format!("Hello {}", name)),
        None => (StatusCode::BAD_REQUEST, "Hello unknown".to_string())
    }
}