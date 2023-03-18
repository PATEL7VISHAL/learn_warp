use crate::_states::{Answer, QuestionId, Store, DB};

pub async fn get_answer(question_id: i32, db: DB) -> Result<impl warp::Reply, warp::Rejection> {
    // let a = true;
    // let res = store.questions.read().await.get(&QuestionId(question_id));
    // let questions = store.questions.read().await;
    // let res = questions.get(&QuestionId(question_id));

    // if let Some(_question) = res {
    // let question = _question.clone();
    //NOTE: we are not using this question in ouput (later we will use)
    // drop(questions); //PF: for now it drop the questions to make free the thread but there is

    // NOTE: what is difference between this two code snippits
    // let res = store
    //     .answers
    //     .read()
    //     .await
    //     .iter()
    //     .map_while(|(_, answer)| answer.question_id.eq(&question.id).then_some(answer))
    //     .collect::<Vec<&Answer>>();

    // let all_answers = store.answers.read().await;
    // let answers = all_answers
    //     .iter()
    //     .filter_map(|(_, answer)| answer.question_id.eq(&question.id).then_some(answer))
    //     .collect::<Vec<_>>();

    // return Ok(warp::reply::json(&answers));
    // }

    // Err(warp::reject::custom(handle_errors::Error::QuestionNotFound))

    let res = db.get_answer(QuestionId(question_id)).await.unwrap();

    // return Ok(warp::reply::json(&answers));
    return Ok(warp::reply::json(&res));
}
