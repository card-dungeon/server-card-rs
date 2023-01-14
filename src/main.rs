use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{doc, Document};
use dotenv;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // build our application with a route
    let app = Router::new().route("/", get(handler));
    let db = init_db().await;
    print!("{:?}", db);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn init_db() -> mongodb::error::Result<()> {
    let client_options = ClientOptions::parse(
        std::env::var("DB_URL").expect("DB 경로가 설정되어야 합니다"),
    )
    .await?;
    let client = Client::with_options(client_options)?;
    let database = client.database("test");
    // let docs = vec![
    //     doc! { "title": "1984", "author": "George Orwell" },
    //     doc! { "title": "Animal Farm", "author": "George Orwell" },
    //     doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    // ];
    // let collection = database.collection::<Document>("books");
    // // Insert some documents into the "mydb.books" collection.
    // collection.insert_many(docs, None).await?;
    for collection_name in database.list_collection_names(None).await? {
        println!("{}", collection_name);
    }
    Ok(())
}