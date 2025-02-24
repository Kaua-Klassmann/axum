use std::{env, sync::Arc};

use argon2::Argon2;
use config::cors::get_cors;
use database::database::get_db_connection;
use dotenvy::dotenv;
use reqwest::Client as ReqwestClient;
use state::AppState;
use tokio::net::TcpListener;

mod config;
mod database;
mod handlers;
mod jwt;
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
        .unwrap();

    let db_conn = get_db_connection().await;
    let argon2 = Arc::new(Argon2::default());
    let reqwest_client = Arc::new(ReqwestClient::new());

    let state = AppState {
        db_conn,
        argon2,
        reqwest_client,
    };

    let app = routes::configure_routes()
        .layer(get_cors())
        .with_state(state);

    println!("Listening on port {}", port);

    axum::serve(listener, app).await.unwrap();
}
