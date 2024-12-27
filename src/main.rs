use axum::Router;
use routes::configure_routes;
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

    let routes: Router = configure_routes()
        .layer((*config::cors::CORS).clone());

    println!("\nServer running in port {}", app_port);

    axum::serve(listener, routes).await.unwrap()
}
