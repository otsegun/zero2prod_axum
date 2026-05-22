use axum::{Form, extract::State, http::StatusCode};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(State(state): State<PgPool>, Form(form_data): Form<FormData>) -> StatusCode {
    //Generate a random unique identifier
    let request_id = Uuid::new_v4();

    // Create an info span
    let request_span = tracing::info_span!(
        "Adding a new subscriber.", 
        %request_id, subsciber_email = %form_data.email,
        subscirber_name = %form_data.name);

    // Enter request span
    let _request_span_guard = request_span.enter();

    //Log subscriber email and name
    tracing::info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber.",
        request_id,
        form_data.email,
        form_data.name
    );
    tracing::info!(
        "request_id {} - Saving new subscriber details in the database",
        request_id
    );

    let query_span = tracing::info_span!("Saving new subscriber details in the database");

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
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - New subscriber details have been saved",
                request_id
            );
            StatusCode::OK
        }
        Err(e) => {
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
