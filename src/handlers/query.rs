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
