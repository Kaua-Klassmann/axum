use tokio::net::TcpListener;

mod config;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    let app = routes::configure_routes()
        .layer((*config::cors::CORS).clone());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on port 3000");

    axum::serve(listener, app).await.unwrap();
}