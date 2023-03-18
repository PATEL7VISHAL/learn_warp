use std::collections::HashMap;

use warp::hyper::StatusCode;

use crate::_states::{Answer, Store, DB};

pub async fn add_answer(
    question_id: i32,
    db: DB,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    //PF: may here  the store is locked but only for signle line because locking for temporary
    let answer = Answer::new(
        //NOTE: IDK why here need clone but move from parm is more efficiant.
        params.get("id").unwrap().clone().parse().unwrap(), //BG: may be need ot handle the error
        params.get("content").unwrap().clone(),
        // params.get("questionId").unwrap().clone(), //NOTE: not need to get from the form
        question_id,
    );

    // let is_question_found = store
    //     .questions
    //     .read()
    //     .await
    //     .get(&answer.question_id)
    //     .is_some();

    // if is_question_found {
    //     let mut answers = store.answers.write().await;
    //     answers.insert(answer.id.clone(), answer);

    // return Ok(warp::reply::with_status("Answer Added", StatusCode::OK));
    // }

    // Err(warp::reject::custom(handle_errors::Error::QuestionNotFound))

    let res = db.add_answer(answer).await;
    println!("{:#?}", res);

    return Ok(warp::reply::with_status("Answer Added", StatusCode::OK));
}
