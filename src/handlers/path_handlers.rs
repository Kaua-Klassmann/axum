use axum::extract::Path;

pub async fn path(Path(name): Path<String>) -> String {
    format!("Hello {}", name)
}