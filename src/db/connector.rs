use mongodb::{Client, options::ClientOptions, Database};

pub async fn init() -> Result<Database, mongodb::error::Error> {
    let client_options = ClientOptions::parse(
        std::env::var("DB_URL").expect("DB 경로가 설정되어야 합니다"),
    ).await?;
    
    let client = Client::with_options(client_options)?;
    let database = client.database("secret-dungeon");

    println!("Success connect Database!");

    Ok(database)
}