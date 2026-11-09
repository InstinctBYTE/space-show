use dioxus::prelude::*;

fn main(){
  dioxus::launch(App);
}


#[component]
fn App() -> Element {
    rsx! {
        div {
            h1 { "Hello, Dioxus!" }
            p { "Application started from main.rs" }
        }
    }
}