use warp::Filter;
use std::collections::HashMap;
use super::handlers;
use mongodb::Client;

pub fn quizzes(client: Client) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_quizzes(client.clone())
        .or(recent_added_quizzes(client.clone()))
        .or(add_quiz(client.clone()))
}

fn with_mongodb(client: Client) -> impl Filter<Extract = (Client,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}

pub fn get_quizzes(client: Client) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("quizzes"))
        .and(warp::query::<HashMap<String, String>>())
        .and(with_mongodb(client))
        .and_then(handlers::quizzes)
}

pub fn recent_added_quizzes(client: Client) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("recent_quizzes"))
        .and(warp::query::<HashMap<String, String>>())
        .and(with_mongodb(client))
        .and_then(handlers::recent_added_quizzes)
}

pub fn add_quiz(client: Client) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("api"))
        .and(warp::path("quizzes"))
        .and(warp::body::json())
        .and(with_mongodb(client))
        .and_then(handlers::add_quiz)
}