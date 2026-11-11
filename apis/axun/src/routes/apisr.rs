use axum::{Json, extract::State};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use models::database::configs::AppState;
use shareds::{LoginRequest, LoginResponse};
use std::sync::Arc;
use surrealdb::engine::any::connect;
use surrealdb::{opt::auth::Record, types::SurrealValue};

#[derive(SurrealValue)]
pub struct Params {
    pub name: String,
    pub pass: String,
}

pub enum AppError {
    Unauthorized,
    Db(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "invalid credentiasl").into_response()
            }
            AppError::Db(m) => (StatusCode::INTERNAL_SERVER_ERROR, m).into_response(),
        }
    }
}

pub async fn login(Json(req): Json<LoginRequest>) -> Result<Json<LoginResponse>, AppError> {
    // One connection per login attempt: signin authenticates the connection itself.
    let db = connect("ws://127.0.0.1:8000")
        .await
        .map_err(|e| AppError::Db(e.to_string()))?;

    // Next step: return this token in a cookie.
    let _token = db
        .signin(Record {
            namespace: "main".to_string(),
            database: "main".to_string(),
            access: "account".to_string(),
            params: Params {
                name: req.name.clone(),
                pass: req.pass,
            },
        })
        .await
        .map_err(|_| AppError::Unauthorized)?;

    Ok(Json(LoginResponse {
        message: format!("welcome {}", req.name),
    }))
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
