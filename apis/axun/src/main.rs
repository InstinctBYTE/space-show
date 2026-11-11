mod routes;
use std::sync::Arc;

use crate::routes::roads::app;
use models::database::configs::AppState;
use routes::roads;
use surrealdb::engine::any::connect;
use surrealdb::opt::auth::Root;
use tokio::net::TcpListener;
use tower_http;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = connect("ws://127.0.0.1:8000").await?;

    db.use_ns("main").use_db("main").await?;
    let state = Arc::new(AppState { db });

    let app = app(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
