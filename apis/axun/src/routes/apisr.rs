use models::database::configs::{self, addd};
pub async fn hallo() -> String {
    let suma = addd();
    format!("hola desde apis con funciones de otros crates{}", suma)
}
