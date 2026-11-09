use models::database::configs::{self, addd};
use axum::Json;
use shareds::{LoginResponse,LoginRequest};
pub async fn hallo() -> String {
    let suma = addd();
    format!("hola desde apis con funciones de otros crates{}", suma)
}

pub async fn login(Json(req):Json<LoginRequest>)->Json<LoginResponse>{
  Json(LoginResponse{
    message:format!("hello {}",req.name),
  })
}