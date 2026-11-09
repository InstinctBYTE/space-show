use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct LoginRequest{
  pub name:String,
  pass:String,
}

#[derive(Serialize,Deserialize)]
pub struct LoginResponse{
  pub message:String,
}