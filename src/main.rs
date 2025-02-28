use std::env;

use dotenvy::dotenv;
use tokio::net::TcpListener;

use crate::app::create_app;

mod app;
mod config;
mod database;
mod handlers;
mod jwt;
mod middlewars;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port: u16 = env::var("APP_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Port is already in use");

    let app = create_app().await;

    println!("Listening on port {}", port);

    axum::serve(listener, app).await.unwrap();
}
