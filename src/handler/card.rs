use axum::{response::IntoResponse, Json, http::StatusCode, extract::State};
use mongodb::{bson::{doc, to_document}, Database, Collection, options::FindOptions, options::UpdateOptions};
use mongodm::operator::*;
use tracing::info;
use serde_json::json;
use tokio_stream::StreamExt;

use crate::model::card::Card;

pub async fn list(State(database): State<Database>) -> impl IntoResponse {
    let coll: Collection<Card> = database
        .collection::<Card>("card");

    let mut options = FindOptions::default();
    options.sort = Some(doc! {
        "id": 1
    });
 
    let mut cursor = coll
        .find(None, options)
        .await
        .expect("could not load card list");
    let mut rows: Vec<Card> = Vec::new();
    // let results: Vec<Result<Document>> = cursor.collect().await;
    while let Some(doc) = cursor.next().await {
        rows.push(doc.expect("could not load listings info."));
    }

    (StatusCode::OK, Json(rows))
}

pub async fn create(
    State(database): State<Database>,
    Json(payload):Json<Card>
) -> impl IntoResponse {
    info!("{:?}", payload);

    let document = to_document(&payload).unwrap();

    let filter = doc! {Or: [{"card_id": payload.card_id.clone()}, {"name": payload.name.clone()}]};
    let insert = doc! {SetOnInsert: document};
    let mut options = UpdateOptions::default();
    options.upsert = Some(true);

    match database.collection::<Card>("card").update_one(
        filter, insert, options
    ).await {
        Ok(_) => (StatusCode::CREATED, Json(json!({"success": true}))),
        Err(e) => {
            tracing::error!("{:?}", e);
            (StatusCode::EXPECTATION_FAILED, Json(json!({"err": {"message": "create card failer"}})))
        },
    }
}

pub async fn create_many(
    State(database): State<Database>,
    Json(payload):Json<Vec<Card>>
)-> impl IntoResponse {
    info!("{:?}", payload);

    match database.collection::<Card>("card").insert_many(payload.clone(), None).await {
        Ok(card) => (StatusCode::CREATED, Json(json!(card))),
        Err(e) => {
            tracing::error!("{:?}", e);
            (StatusCode::EXPECTATION_FAILED, Json(json!({"err": {"message": "create card failer"}})))
        },
    }
}