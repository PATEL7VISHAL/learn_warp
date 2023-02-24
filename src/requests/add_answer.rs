use std::collections::HashMap;

use warp::hyper::StatusCode;

use crate::{
    MyError,
    _states::{Answer, Store},
};

pub async fn add_answer(
    store: Store,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    //PF: may here  the store is locked but only for signle line because locking for temporary
    let answer = Answer::new(
        //NOTE: IDK why here need clone but move from parm is more efficiant.
        params.get("id").unwrap().clone(),
        params.get("content").unwrap().clone(),
        params.get("questionId").unwrap().clone(),
    );

    let is_question_found = store
        .questions
        .read()
        .await
        .get(&answer.question_id)
        .is_some();

    if is_question_found {
        let mut answers = store.answers.write().await;
        answers.insert(answer.id.clone(), answer);

        return Ok(warp::reply::with_status("Answer Added", StatusCode::OK));
    }

    Err(warp::reject::custom(MyError::QuestionNotFound))
}
