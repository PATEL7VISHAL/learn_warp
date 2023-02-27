use crate::_states::{Question, QuestionId, Store};
use std::collections::HashMap;
use warp::hyper::StatusCode;

pub async fn update_function(
    id: String,
    mut store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(id)) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(handle_errors::Error::QuestionNotFound)),
    }

    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}
