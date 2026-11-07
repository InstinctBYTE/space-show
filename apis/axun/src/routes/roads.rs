use crate::routes::apisr::*;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
pub fn app() -> Router {
    Router::new().route("/api", get(hallo))
}
