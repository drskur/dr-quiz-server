use warp::Filter;
use super::handlers;
use mongodb::Client;
use crate::models::{RecentAddedQuizzesRequest, QuizzesRequest, NextQuizRequest};

pub fn quizzes(client: Client) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    next_quiz(client.clone())
        .or(get_quiz_yaml(client.clone()))
        .or(get_quizzes(client.clone()))
        .or(recent_added_quizzes(client.clone()))
        .or(add_quiz_yaml(client.clone()))
        .or(add_quiz(client.clone()))
        .or(put_quiz_yaml(client.clone()))
        .or(put_quiz(client.clone()))
        .or(delete_quiz(client))
}

fn with_mongodb(client: Client) -> impl Filter<Extract = (Client,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}

pub fn get_quizzes(client: Client) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("quizzes"))
        .and(warp::query::<QuizzesRequest>())
        .and(with_mongodb(client))
        .and_then(handlers::quizzes)
}

pub fn get_quiz_yaml(client: Client) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("api" / "quizzes" / String / "yaml"))
        .and(with_mongodb(client))
        .and_then(handlers::quiz_yaml)
}

pub fn recent_added_quizzes(client: Client) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("recent_quizzes"))
        .and(warp::query::<RecentAddedQuizzesRequest>())
        .and(with_mongodb(client))
        .and_then(handlers::recent_added_quizzes)
}

pub fn add_quiz(client: Client) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("api" / "quizzes"))
        .and(warp::body::json())
        .and(with_mongodb(client))
        .and_then(handlers::add_quiz)
}

pub fn add_quiz_yaml(client: Client) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("api" / "quizzes" / "yaml"))
        .and(warp::body::json())
        .and(with_mongodb(client))
        .and_then(handlers::add_quiz_yaml)
}

pub fn next_quiz(client: Client) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("api" / "quizzes" / "next"))
        .and(warp::query::<NextQuizRequest>())
        .and(with_mongodb(client))
        .and_then(handlers::next_quiz)
}

pub fn put_quiz(client: Client) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::put()
        .and(warp::path!("api" / "quizzes" / String))
        .and(warp::body::json())
        .and(with_mongodb(client))
        .and_then(handlers::put_quiz)
}

pub fn put_quiz_yaml(client: Client) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::put()
        .and(warp::path!("api" / "quizzes" / String / "yaml"))
        .and(warp::body::json())
        .and(with_mongodb(client))
        .and_then(handlers::put_quiz_yaml)
}

pub fn delete_quiz(client: Client) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::delete()
        .and(warp::path!("api" / "quizzes" / String))
        .and(with_mongodb(client))
        .and_then(handlers::delete_quiz)
}