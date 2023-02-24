use warp::{
    cors::CorsForbidden, filters::body::BodyDeserializeError, hyper::StatusCode, Rejection,
};

use crate::MyError;

pub async fn return_error(r: Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<MyError>() {
        match error {
            &MyError::QuestionNotFound => Ok(warp::reply::with_status(
                error.to_string(),
                StatusCode::RANGE_NOT_SATISFIABLE,
            )),

            _ => Ok(warp::reply::with_status(
                error.to_string(),
                StatusCode::NOT_FOUND,
            )),
        }
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route Not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
