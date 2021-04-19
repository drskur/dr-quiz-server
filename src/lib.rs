use mongodb::options::ClientOptions;
use mongodb::{Client, Collection};
use crate::models::Quiz;

pub mod handlers;
pub mod models;
pub mod filters;

pub async fn get_mongodb_client() -> mongodb::error::Result<Client> {
    let mut opt = ClientOptions::parse("mongodb://drskur:hNkEVxtL0hzrSukhZnNXktU4lmlzWFtoWs2MlGykDhdnQlMfEv9AYxwbeYrDDDlx5DgxqePR1Cx8Xyy5zNYh3A==@drskur.mongo.cosmos.azure.com:10255/?ssl=true&replicaSet=globaldb&retrywrites=false&maxIdleTimeMS=120000&appName=@drskur@")
        .await?;
    let client = Client::with_options(opt)?;

    Ok(client)
}

pub fn get_quiz_collection(client: mongodb::Client) -> Collection<Quiz> {
    let db = client.database("drquiz");
    let coll = db.collection("quizzes");

    coll
}