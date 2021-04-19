use std::convert::Infallible;
use std::collections::HashMap;
use crate::models::{Quiz};
use mongodb::{Client, bson};
use mongodb::bson::doc;
use futures::stream::StreamExt;
use crate::get_quiz_collection;
use mongodb::options::FindOptions;

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

pub async fn recent_added_quizzes(p: HashMap<String, String>, client: Client) -> Result<impl warp::Reply, Infallible> {

    let coll = get_quiz_collection(client);

    let find_options = FindOptions::builder()
        .limit(10)
        .sort(doc!{ "_id": -1 })
        .build();

    let cursor = coll.find(doc!{  }, Some(find_options)).await
        .expect("cannot find documents");

    let docs = cursor
        .map(|r| r.unwrap())
        .collect::<Vec<_>>()
        .await;

    Ok(warp::reply::json(&docs))
}

pub async fn add_quiz(mut new_quiz: Quiz, client: Client) -> Result<impl warp::Reply, Infallible> {

    let coll = get_quiz_collection(client);

    new_quiz._id = Some(bson::oid::ObjectId::new());
    let result = coll.insert_one(new_quiz, None).await
        .expect("cannot insert quiz");

    let quiz = coll.find_one(doc!{ "_id": result.inserted_id }, None).await
        .expect("cannot find inserted quiz");

    Ok(warp::reply::json(&quiz))
}