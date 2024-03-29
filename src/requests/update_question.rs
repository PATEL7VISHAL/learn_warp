use crate::_states::{Question, QuestionId, Store, DB};
use std::collections::HashMap;
use warp::hyper::StatusCode;

pub async fn update_function(
    id: i32,
    db: DB,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    // match store.questions.write().await.get_mut(&QuestionId(id)) {
    //     Some(q) => *q = question,
    //     None => return Err(warp::reject::custom(handle_errors::Error::QuestionNotFound)),
    // }

    let res = db.update_question(QuestionId(id), question).await;
    println!("res: {:#?}", res);

    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}
