use std::convert::Infallible;
use std::collections::HashMap;
use warp::http::Response;

pub async fn hello(p: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
    match p.get("name") {
        Some(name) => Ok(warp::reply::html(format!("Hello, {}. This HTTP triggered function executed successfully.", name))),
        None => Ok(warp::reply::html(String::from("This HTTP triggered function executed successfully. Pass a name in the query string for a personalized response."))),
    }

}