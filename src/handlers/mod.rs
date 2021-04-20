use std::convert::Infallible;
use crate::models::{Quiz, RecentAddedQuizzesRequest, QuizRequest, QuizzesRequest};
use mongodb::{Client, bson};
use mongodb::bson::{doc, Document};
use futures::stream::StreamExt;
use crate::{get_collection};
use mongodb::options::{FindOptions};
use std::str::FromStr;

pub async fn quizzes(req: QuizzesRequest, client: Client) -> Result<impl warp::Reply, Infallible> {
    let coll = get_collection::<Quiz>(client, "quizzes");

    let find_options = FindOptions::builder()
        .limit(req.limit.unwrap_or_else(|| 10))
        .build();

    let cursor = coll.find(doc! {  }, Some(find_options)).await
        .expect("cannot find documents");

    let docs = cursor
        .map(|r| r.unwrap())
        .collect::<Vec<_>>()
        .await;

    Ok(warp::reply::json(&docs))
}

pub async fn recent_added_quizzes(req: RecentAddedQuizzesRequest, client: Client) -> Result<impl warp::Reply, Infallible> {
    let coll = get_collection::<Quiz>(client, "quizzes");

    let find_options = FindOptions::builder()
        .limit(req.limit.unwrap_or_else(|| 10))
        .sort(doc! { "_id": -1 })
        .build();

    let cursor = coll.find(doc! {  }, Some(find_options)).await
        .expect("cannot find documents");

    let docs = cursor
        .map(|r| r.unwrap())
        .collect::<Vec<_>>()
        .await;

    Ok(warp::reply::json(&docs))
}

pub async fn add_quiz(new_quiz: QuizRequest, client: Client) -> Result<impl warp::Reply, Infallible> {
    let coll = get_collection::<Document>(client, "quizzes");

    let doc = bson::to_document(&new_quiz).unwrap();
    let result = coll.insert_one(doc, None).await
        .expect("cannot insert quiz");

    Ok(warp::reply::json(&result))
}

pub async fn put_quiz(id: String, quiz: QuizRequest, client: Client) -> Result<impl warp::Reply, Infallible> {
    let coll = get_collection::<Quiz>(client, "quizzes");
    let update = doc! {
        "$set": bson::to_document(&quiz).unwrap()
    };

    let oid = bson::oid::ObjectId::from_str(&id).unwrap();
    let result = coll.update_one(doc! { "_id": oid }, update, None).await
        .expect("cannot update quiz");

    Ok(warp::reply::json(&result))
}

pub async fn delete_quiz(id: String, client: Client) -> Result<impl warp::Reply, Infallible> {
    let coll = get_collection::<Quiz>(client, "quizzes");
    let oid = bson::oid::ObjectId::from_str(&id).unwrap();
    let result = coll.delete_one(doc! { "_id": oid }, None).await
        .expect("cannot delete quiz");

    Ok(warp::reply::json(&result))
}