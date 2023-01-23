use axum::{routing::get, Router};
use std::net::SocketAddr;

use dotenv;
use tracing_subscriber::FmtSubscriber;
use tracing::{Level};

mod routes;
mod db;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database = db::connector::init().await.unwrap();

    let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .finish();
    tracing::subscriber::set_global_default(subscriber)
    .expect("setting default subscriber failed");

    let app = Router::new().route("/list", get(routes::card::list));
    let app = app.with_state(database);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
