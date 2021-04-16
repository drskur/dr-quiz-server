use std::convert::Infallible;
use std::collections::HashMap;
use crate::models::{AddQuizRequest, Quiz};
use mongodb::options::ClientOptions;
use mongodb::{Client, Database, Collection, bson, bson::Document};
use mongodb::bson::doc;
use futures::stream::StreamExt;
use futures::FutureExt;

async fn get_quiz_collection() -> mongodb::error::Result<Collection<Quiz>> {
    let mut opt = ClientOptions::parse("mongodb://drskur:hNkEVxtL0hzrSukhZnNXktU4lmlzWFtoWs2MlGykDhdnQlMfEv9AYxwbeYrDDDlx5DgxqePR1Cx8Xyy5zNYh3A==@drskur.mongo.cosmos.azure.com:10255/?ssl=true&replicaSet=globaldb&retrywrites=false&maxIdleTimeMS=120000&appName=@drskur@")
        .await?;
    let client = Client::with_options(opt)?;
    let db = client.database("drquiz");
    let coll = db.collection("quizzes");

    Ok(coll)
}

async fn get_quiz_document_collection() -> mongodb::error::Result<Collection<Document>> {
    let mut opt = ClientOptions::parse("mongodb://drskur:hNkEVxtL0hzrSukhZnNXktU4lmlzWFtoWs2MlGykDhdnQlMfEv9AYxwbeYrDDDlx5DgxqePR1Cx8Xyy5zNYh3A==@drskur.mongo.cosmos.azure.com:10255/?ssl=true&replicaSet=globaldb&retrywrites=false&maxIdleTimeMS=120000&appName=@drskur@")
        .await?;
    let client = Client::with_options(opt)?;
    let db = client.database("drquiz");
    let coll = db.collection("quizzes");

    Ok(coll)
}

pub async fn quizzes(p: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
    let coll = get_quiz_collection().await
        .expect("cannot open collection");

    let cursor = coll.find(doc!{  }, None).await
        .expect("cannot find documents");

    let docs = cursor
        .map(|r| r.unwrap())
        .collect::<Vec<_>>()
        .await;

    Ok(warp::reply::json(&docs))
}

pub async fn add_quiz(req: AddQuizRequest) -> Result<impl warp::Reply, Infallible> {

    let coll = get_quiz_collection().await
        .expect("cannot open collection");

    let new_quiz = Quiz {
        _id: bson::oid::ObjectId::new(),
        subject: req.subject.clone()
    };

    coll.insert_one(new_quiz, None).await
        .expect("cannot insert quiz");

    Ok(warp::reply::json(&req))
}