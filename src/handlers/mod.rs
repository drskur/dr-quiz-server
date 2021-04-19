use std::convert::Infallible;
use std::collections::HashMap;
use crate::models::{AddQuizRequest, Quiz};
use mongodb::{Client, bson};
use mongodb::bson::doc;
use futures::stream::StreamExt;
use futures::FutureExt;
use crate::get_quiz_collection;


pub async fn quizzes(p: HashMap<String, String>, client: Client) -> Result<impl warp::Reply, Infallible> {

    let coll = get_quiz_collection(client);

    let cursor = coll.find(doc!{  }, None).await
        .expect("cannot find documents");

    let docs = cursor
        .map(|r| r.unwrap())
        .collect::<Vec<_>>()
        .await;

    Ok(warp::reply::json(&docs))
}

pub async fn add_quiz(req: AddQuizRequest, client: Client) -> Result<impl warp::Reply, Infallible> {

    let coll = get_quiz_collection(client);

    let new_quiz = Quiz {
        _id: bson::oid::ObjectId::new(),
        subject: req.subject.clone()
    };

    coll.insert_one(new_quiz, None).await
        .expect("cannot insert quiz");

    Ok(warp::reply::json(&req))
}