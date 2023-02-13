#![allow(unused, dead_code)]

use std::str::FromStr;

use warp::{
    body::json,
    hyper::{Method, StatusCode},
    reject::Reject,
    Filter, Rejection,
};
pub mod _states;
use _states::*;

#[derive(Debug)]
pub struct InvalidId;
// pub struct InvalidId{}; //NOTE: What is this differance between above type and this?
impl Reject for InvalidId {}

async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId::from_str("12").expect("No id provided"),
        "First question".to_owned(),
        "Content of the question".to_owned(),
        Some(vec!["tags".to_owned()]),
    );

    if let Ok(_) = question.id.0.parse::<i32>() {
        Ok((warp::reply::json(&question)))
    } else {
        Err(warp::reject::custom(InvalidId))
    }
}

async fn return_error(r: Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}",r);

    if let Some(_) = r.find::<InvalidId>() {
        Ok(warp::reply::with_status(
            "No valid ID presented",
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route Not found",
            StatusCode::NOT_FOUND,
        ))
    }
}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        // .allow_header("not-in-the-request")
        .allow_header("content-type")
        .allow_methods(&[ Method::POST, Method::DELETE, Method::GET]);

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions)
        .recover(return_error); //NOTE: if any error oucar in above routes then it's called in we
                                // can handle the errors here

    let routes = get_items.with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
