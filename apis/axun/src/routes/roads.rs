use crate::routes::apisr::*;
use axum::{
    Json, Router,
    extract::State,
    routing::{delete, get, post, put},
};
use models::database::configs::AppState;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub fn app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api", get(db_health))
        .route("/api/login", post(login))
        .layer(CorsLayer::permissive())
        .with_state(state)
}
