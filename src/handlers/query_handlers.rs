use std::collections::HashMap;

use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse
};

pub async fn query(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    match params.get("name") {
        Some(name) => (StatusCode::OK, format!("Hello {}", name)),
        None => (StatusCode::BAD_REQUEST, format!("Hello unknown"))
    }
}