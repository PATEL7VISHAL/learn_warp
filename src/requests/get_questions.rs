use crate::_states::{Pagination, Question, Store, DB};
use handle_errors::Error;
use std::{collections::HashMap, sync::Arc};

// async fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
fn _extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if let (Some(value_s), Some(value_e)) = (params.get("start"), params.get("end")) {
        return Ok(Pagination {
            start: value_s.parse().map_err(Error::ParseError)?,
            // start: value_s.parse().map_err(|e| Error::ParseError(e)).unwrap(),
            end: value_e.parse().map_err(Error::ParseError)?,
        });
    }

    Err(Error::MissingParameters)
}
pub async fn get_questions(
    params: HashMap<String, String>,
    db: DB,
    request_id: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    // let res : Vec<Question> = store.questions.values().collect();
    log::info!("request_id: {}", request_id);

    if params.is_empty() {
        let question = db.get_question(None).await.unwrap();
        Ok(warp::reply::json(&question))
    } else {
        let pagination = _extract_pagination(params)?;

        //        // let res = &&res[pagination.start .. pagination.end];
        //        //NOTE: IDK why here the double ref is required
        //        let max_len = res.len();
        //        if pagination.start >= max_len
        //            || pagination.end >= max_len
        //            || pagination.start > pagination.end
        //        {
        //            return Err(warp::reject::custom(Error::QuestionNotFound));
        //        }

        //        let res = &res[pagination.start - 1..pagination.end];

        // Ok(warp::reply::json(&res))
        Ok(warp::reply::json(&"sdf".to_string()))
    }
}
