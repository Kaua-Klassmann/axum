use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Payload {
    name: String
}

#[derive(Serialize)]
struct Response {
    name: String,
    has_3_or_more_letters: bool
}

pub async fn json(
    Json(payload): Json<Payload>
) -> impl IntoResponse {
    let response = Response {
        name: payload.name.clone(),
        has_3_or_more_letters: payload.name.chars().count() > 2
    };

    (StatusCode::OK, Json(response))
}