#![allow(unused, dead_code)]

use std::str::FromStr;

use warp::{body::json, Filter};
pub mod _states;
use _states::*;

async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First question".to_owned(),
        "Content of the question".to_owned(),
        Some(vec!["tags".to_owned()]),
    );

    Ok(warp::reply::json(&question))
}

#[tokio::main]
async fn main() {
    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions);

    let routes = get_items;

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
