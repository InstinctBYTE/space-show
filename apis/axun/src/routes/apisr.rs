use axum::{Json, extract::State};
use models::database::configs::AppState;
use shareds::{LoginRequest, LoginResponse};
use std::sync::Arc;
pub async fn login(Json(req): Json<LoginRequest>) -> Json<LoginResponse> {
    Json(LoginResponse {
        message: format!("hello {}", req.name),
    })
}

pub async fn db_health(State(state): State<Arc<AppState>>) -> Result<String, String> {
    let mut res = state
        .db
        .query("RETURN 1")
        .await
        .map_err(|e| e.to_string())?;
    let n: Option<i64> = res.take(0).map_err(|e| e.to_string())?;
    Ok(format!("db ok: {n:?}"))
}
