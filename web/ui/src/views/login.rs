use dioxus::prelude::*;
use shareds::{LoginResponse,LoginRequest};
#[component]
pub fn LoginView()->Element{
  let mut name= use_signal(String::new);
  let mut pass= use_signal(String::new);
  let mut message= use_signal(String::new)
}