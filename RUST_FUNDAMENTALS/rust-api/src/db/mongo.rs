use mongodb::{Client,Database}
use std::env;

pub async fn get_db() -> Database{
    let uri = env::var("MONGO_URI").expect("MONGO CONNECTION STRING IS MISSING.")
    let db_name = env::var("DB_NAME").expect("DB NAME IS MISSING.")

    let client = Client::with_uri_str(uri).await.unwrap();
    client.database(&db_name)
}