use axum::{Form, extract::State, http::StatusCode};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(State(state): State<PgPool>, Form(form_data): Form<FormData>) -> StatusCode {
    //Log subscriber email and name
    log::info!(
        "Adding '{}' '{}' as a new subscriber.",
        form_data.email,
        form_data.name
    );
    log::info!("Saving new subscirber details in the database");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form_data.email,
        form_data.name,
        Utc::now()
    )
    .execute(&state)
    .await
    {
        Ok(_) => {
            log::info!("New subscriber details have been saved");
            StatusCode::OK
        }
        Err(e) => {
            log::error!("Failed to execute query: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
