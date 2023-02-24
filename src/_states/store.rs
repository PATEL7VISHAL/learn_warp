use super::{Answer, AnswerId, Question, QuestionId};
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::sync::Arc;
use std::{collections::HashMap, str::FromStr};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Store {
    pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            questions: Arc::new(RwLock::new(HashMap::new())),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn init(&mut self) {
        let mut _store = Self::new();

        // let que1 = Question::new(
        //     QuestionId::from_str("1").expect("No ID Provided"),
        //     "brave browser".to_owned(),
        //     "it's better choise compaire to the chrome".to_owned(),
        //     Some(vec!["#brave".to_owned()]),
        // );
        // _store.add_question(que1);

        let file = include_str!("../../questions.json");
        let questions: HashMap<QuestionId, Question> =
            serde_json::from_str(file).expect("can't read the questions.json");

        let mut handle = self.questions.write().await;

        *handle = questions; //PF: here is some issue about the older questoins member
                             //memory is allocated in self::new() and gose to deallocate
                             //at here (because of moving) to not use of the memory
    }

    pub async fn add_question(&mut self, question: Question) {
        self.questions
            .write()
            .await
            .insert(question.id.clone(), question);
    }
}
