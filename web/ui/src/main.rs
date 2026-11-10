use dioxus::prelude::*;
use shareds::{LoginResponse,LoginRequest};
mod views;
use crate::views::login::LoginView;

fn main(){
  dioxus::launch(login)
}