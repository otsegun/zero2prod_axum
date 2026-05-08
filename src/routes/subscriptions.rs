use axum::{Form, http::StatusCode};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(Form(_form_data): Form<FormData>) -> StatusCode {
    StatusCode::OK
}
