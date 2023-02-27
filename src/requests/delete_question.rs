use crate::_states::{Question, QuestionId, Store};
use std::collections::HashMap;
use warp::hyper::StatusCode;

pub async fn delete_question(
    id: String,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let id = QuestionId(id);
    let mut questions = store.questions.write().await;

    //NOTE: in this commented it's create the dead lock and unable to porcess
    // if let Some(_) = store.questions.read().await.get(&id) { b
    // store.questions.write().await.remove(&id);

    if let Some(_) = questions.get(&id) {
        questions.remove(&id);
        Ok(warp::reply::with_status("Question Deleted", StatusCode::OK))
    } else {
        Err(warp::reject::custom(handle_errors::Error::QuestionNotFound))
    }
}
