use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct LoginRequest{
  pub name:String,
  pub pass:String,
}

#[derive(Serialize,Deserialize)]
pub struct LoginResponse{
  pub message:String,
}