use dioxus::prelude::*;
use shareds::{LoginRequest, LoginResponse};
#[component]
pub fn LoginView() -> Element {
    let mut name = use_signal(String::new);
    let mut pass = use_signal(String::new);
    let mut message = use_signal(String::new);

    let on_click = move |_| {
        spawn(async move {
            let body = LoginRequest {
                name: name(),
                pass: pass(),
            };
            message.set(match send_login(body).await {
                Ok(r) => r.message,
                Err(e) => format!("error:{e}"),
            });
        });
    };

    rsx! {
        input {value:"{name}",oninput:move |e| name.set(e.value())}
        input {r#type:"password",value:"{pass}",oninput:move |e| pass.set(e.value())}
        button {onclick:on_click,"login"}
        p{"{message}"}
    }
}

pub async fn send_login(body: LoginRequest) -> Result<LoginResponse, reqwest::Error> {
    reqwest::Client::new()
        .post("http://localhost:3000/api/login")
        .json(&body)
        .send()
        .await?
        .json::<LoginResponse>()
        .await
}
