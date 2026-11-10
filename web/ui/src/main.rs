use dioxus::prelude::*;
use shareds::{LoginRequest, LoginResponse};
mod views;
use crate::views::login::LoginView;

fn main() {
    dioxus::launch(LoginView);
}
