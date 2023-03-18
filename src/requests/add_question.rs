use crate::_states::{Question, Store, DB};
use std::collections::HashMap;
use warp::hyper::StatusCode;

pub async fn add_question(
    mut db: DB,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    // let is_founed = store.questions.read().await.get(&question.id).is_some();

    // if !is_founed {
    //     store.add_question(question).await;

    //     return Ok(warp::reply::with_status(
    //         "Question Added".to_string(),
    //         StatusCode::OK,
    //     ));
    // }

    // Err(warp::reject::custom(
    //     handle_errors::Error::QustionAlreadyExist,
    // ))

    let res = db.add_question(question).await.unwrap();

    Ok(warp::reply::json(&res))
}
