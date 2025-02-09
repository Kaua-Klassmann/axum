use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse
};

pub async fn path(
    Path(name): Path<String>
) -> impl IntoResponse {
    (StatusCode::OK, format!("Hello {}", name))
}