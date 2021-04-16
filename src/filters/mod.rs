use warp::Filter;
use std::collections::HashMap;
use super::handlers;

pub fn quizzes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_quizzes()
        .or(add_quiz())
}

pub fn get_quizzes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("quizzes"))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(handlers::quizzes)
}

pub fn add_quiz() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("api"))
        .and(warp::path("quizzes"))
        .and(warp::body::json())
        .and_then(handlers::add_quiz)
}