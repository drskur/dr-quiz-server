use warp::Filter;
use std::collections::HashMap;
use super::handlers;

pub fn hello() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("quizzes"))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(handlers::hello)
}