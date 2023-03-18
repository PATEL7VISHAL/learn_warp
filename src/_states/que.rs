use serde::{Deserialize, Serialize};
pub use std::io::{Error, ErrorKind};
pub use std::str::FromStr;
use surrealdb::sql::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct QuestionId(pub i32);

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
            false => Ok(QuestionId(id.parse().unwrap())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provider")),
        }
    }
}

//PF: note here is idk about the Into trait so corrently use the direct methdo
impl QuestionId {
    pub fn into(self) -> Value {
        self.0.into()
    }
}
