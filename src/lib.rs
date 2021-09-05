use mongodb::options::ClientOptions;
use mongodb::{Client, Collection};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

pub mod handlers;
pub mod models;
pub mod filters;

pub async fn get_mongodb_client() -> mongodb::error::Result<Client> {
    let mongo_url = std::env::var("MONGO_URL").expect("MONGO_URL must be set");
    let opt = ClientOptions::parse(&mongo_url)
        .await?;
    let client = Client::with_options(opt)?;

    Ok(client)
}

pub fn get_collection<T: Serialize + DeserializeOwned + Unpin + Debug>(client: mongodb::Client, coll_name: &str) -> Collection<T> {
    let db = client.database("drquiz");
    db.collection(coll_name)
}
