use routes::configure_routes;
use tokio::net::TcpListener;

mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, configure_routes()).await.unwrap()
}