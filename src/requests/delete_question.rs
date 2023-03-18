use crate::_states::{Question, QuestionId, Store, DB};
use std::collections::HashMap;
use warp::hyper::StatusCode;

pub async fn delete_question(
    id: i32,
    db: DB,
) -> Result<impl warp::Reply, warp::Rejection> {
    let id = QuestionId(id);
    // let mut questions = store.questions.write().await;

    //NOTE: in this commented it's create the dead lock and unable to porcess
    // if let Some(_) = store.questions.read().await.get(&id) { b
    // store.questions.write().await.remove(&id);

    // if questions.get(&id).is_some(){
    //     questions.remove(&id);
    //     Ok(warp::reply::with_status("Question Deleted", StatusCode::OK))
    // } else {
    //     Err(warp::reject::custom(handle_errors::Error::QuestionNotFound))
    // }

    let res = db.remove_question(id).await;
    println!("res: {:#?}", res);

    Ok(warp::reply::with_status("Question Deleted", StatusCode::OK))
}
