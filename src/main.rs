use std::env;

use config::{
    cors::get_cors,
    //database::get_db
};
use dotenvy::dotenv;
use tokio::net::TcpListener;

mod config;
mod handlers;
mod routes;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    dotenv().ok();

    //let db = get_db().await;

    let port: u32 = env::var("APP_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let app = routes::configure_routes()
        .layer(get_cors().await);

    let listener = TcpListener::bind(
        format!("0.0.0.0:{}", port)
    ).await.unwrap();

    println!("Listening on port {}", port);

    axum::serve(listener, app).await.unwrap();
}