use serde::{Deserialize, Serialize};
pub use std::io::{Error, ErrorKind};
pub use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionId(String);

impl Question {
    pub fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Self {
            id,
            title,
            content,
            tags,
        }
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_owned())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provider")),
        }
    }
}
