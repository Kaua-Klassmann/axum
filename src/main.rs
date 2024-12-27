use axum::Router;
use migration::MigratorTrait;
use routes::configure_routes;
use sea_orm::{Database, DatabaseConnection};
use tokio::net::TcpListener;

mod config;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    let app_port: u16 = dotenvy::var("APP_PORT")
        .ok()
        .and_then(|p: String| p.parse().ok())
        .unwrap_or(3000);

    let address: String = format!("0.0.0.0:{}", app_port);

    let listener: TcpListener = TcpListener::bind(address).await.unwrap();

    let db: DatabaseConnection = Database::connect((*config::database::DB_OPTIONS).clone())
        .await.expect("Failed to connect on database");
    migration::Migrator::up(&db, None).await.unwrap();

    let routes: Router = configure_routes()
        .layer((*config::cors::CORS).clone());

    println!("\nServer running in port {}", app_port);

    axum::serve(listener, routes).await.unwrap()
}
