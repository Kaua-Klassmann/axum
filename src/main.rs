use std::env;

use dotenvy::dotenv;
use tokio::net::TcpListener;

mod config;
mod database;
mod state;
mod handlers;
mod routes;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port: u16 = env::var("APP_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let app = routes::configure_routes().await;

    let listener = TcpListener::bind(
        format!("0.0.0.0:{}", port)
    ).await.unwrap();

    println!("Listening on port {}", port);

    axum::serve(listener, app).await.unwrap();
}