use super::QuestionId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnswerId(pub i32);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}

impl Answer {
    pub fn new(id: i32, content: String, question_id: i32) -> Self {
        Self {
            id: AnswerId(id),
            content,
            question_id: QuestionId(question_id),
        }
    }
}
