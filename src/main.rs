use std::env;
use warp::{Filter};
use std::net::Ipv4Addr;

use lib::filters;

#[tokio::main]
async fn main() {

    let mongodb_client = lib::get_mongodb_client().await
        .expect("cannot create mongodb client");

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    let routes = filters::quizzes(mongodb_client).with(warp::log("drquiz"));

    warp::serve(routes).run((Ipv4Addr::UNSPECIFIED, port)).await
}
