#[derive(Debug)]
pub enum MyError {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
    QustionAlreadyExist,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ParseError(ref err) => write!(f, "Can't parse the value : {}", err),
            Self::MissingParameters => write!(f, "Missing parameter"),
            Self::QuestionNotFound => write!(f, "Question Not found"),
            Self::QustionAlreadyExist => write!(f, "Question is Already Exist"),
        }
    }
}

impl warp::reject::Reject for MyError {}
