use axum::{response::IntoResponse, Json, http::StatusCode, extract::State};
use mongodb::Database;
use tracing::info;
use serde_json::json;

use crate::model::card::Card;

/// imitating an API response
// #[allow(clippy::unused_async)]
pub async fn list(State(database): State<Database>) -> impl IntoResponse {
    
    // (StatusCode::CREATED, Json(card))
    (StatusCode::OK, Json(json!({"result": "123"})))
}