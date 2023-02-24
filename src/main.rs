#![allow(unused, dead_code)]
use std::collections::HashMap;
use std::str::FromStr;

use warp::{
    body::json,
    cors::CorsForbidden,
    hyper::{Method, StatusCode},
    reject::Reject,
    Filter, Rejection,
};

pub mod _states;
use _states::*;

pub mod requests;
pub use requests::*;

pub mod my_errors;
pub use my_errors::MyError;

pub mod utils;
pub use utils::*;

#[derive(Debug)]
pub struct InvalidId;
// pub struct InvalidId{}; //NOTE: What is this differance between above type and this?
impl Reject for InvalidId {}

#[tokio::main]
async fn main() {
    let mut store = Store::new();
    store.init().await;

    //NOTE: here the warp:any() mean this store data can goes to any filter (but question any
    //filter which are defined or any filter send which we got for the request) may be any filter
    //which we are created.
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        // .allow_header("not-in-the-request")
        .allow_header("content-type")
        .allow_methods(&[Method::POST, Method::DELETE, Method::GET]);

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query()) //NOTE: that the order or add.
        .and(store_filter.clone())
        .and_then(get_questions::get_questions);
    // .recover(return_error); //NOTE: if any error oucar in above routes then it's called in we

    // can handle the errors here
    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(add_question:: add_question);
    // .recover(return_error); //NOTE: make it one common error handeler

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(update_question::update_function);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(delete_question::delete_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(add_answer::add_answer);

    let routes = get_items
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .or(add_answer)
        .with(cors)
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
