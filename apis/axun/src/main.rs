mod routes;
use crate::routes::roads::app;
use routes::roads;
use tokio::net::TcpListener;
use tower_http;
#[tokio::main]
async fn main() {
    let app = app();

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
