use routes::configure_routes;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    let cors: CorsLayer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app_port: u16 = dotenvy::var("APP_PORT")
        .ok()
        .and_then(|p: String| p.parse().ok())
        .unwrap_or(3000);

    let address: String = format!("0.0.0.0:{}", app_port);

    let listener: TcpListener = TcpListener::bind(address).await.unwrap();

    let routes = configure_routes()
        .layer(cors);

    println!("\nServer running in port {}", app_port);

    axum::serve(listener, routes).await.unwrap()
}
