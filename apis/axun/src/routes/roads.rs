use crate::routes::apisr::*;
use axum::{
    routing::{delete, get, post, put},
    Router,Json,
};
use tower_http::cors::CorsLayer;

pub fn app() -> Router {
    Router::new()
  .route("/api", get(hallo))
  .route("/api/login",post(login))
  .layer(CorsLayer::permissive())
}

