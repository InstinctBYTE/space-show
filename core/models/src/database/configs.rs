use std::sync::Arc;

use surrealdb::{
    Surreal,
    engine::any::{Any, connect},
};

pub struct AppState {
    pub db: Surreal<Any>,
}
