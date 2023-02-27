use warp::{
    cors::CorsForbidden, filters::body::BodyDeserializeError, hyper::StatusCode, Rejection,
};

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
    QustionAlreadyExist,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ParseError(ref err) => write!(f, "Can't parse the value : {}", err),
            Self::MissingParameters => write!(f, "Missing parameter"),
            Self::QuestionNotFound => write!(f, "Question Not found"),
            Self::QustionAlreadyExist => write!(f, "Question is Already Exist"),
        }
    }
}

impl warp::reject::Reject for Error {}

pub async fn return_error(r: Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<Error>() {
        match error {
            &Error::QuestionNotFound => Ok(warp::reply::with_status(
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
