use std::collections::BTreeMap;
use std::error::Error;
use std::sync::Arc;
use surrealdb::sql::{thing, Datetime, Object, Thing, Value};
use surrealdb::{Datastore, Response, Session};

use crate::utils::macros::map;

use super::{Answer, AnswerId, Pagination, Question, QuestionId};

#[derive(Clone)]
pub struct DB {
    pub ds: Arc<Datastore>,
    pub sesh: Session,
}

impl DB {
    pub async fn execute(
        &self,
        query: &str,
        vars: Option<BTreeMap<String, Value>>,
    ) -> Result<Vec<Response>, Box<dyn Error>> {
        //NOTE: For currently i took the String if error oucar.
        let res = self.ds.execute(query, &self.sesh, vars, false).await?; //NOTE: may be here we ignore the error handeling.

        Ok(res)
    }

    pub async fn get_question(
        &self,
        pagging: Option<Pagination>,
    ) -> Result<Vec<Response>, Box<dyn Error>> {
        let sql = "SELECT * from questions";
        let res = self.execute(sql, None).await;
        res
    }

    pub async fn add_question(&self, question: Question) -> Result<Vec<Response>, Box<dyn Error>> {
        let sql = "CREATE questions set qId = $qId, title=$title, content=$content, tags = $tags;";
        let tags = question.tags.unwrap_or_default();
        let vars = map![
            "qId".into() => question.id.into(),
            "title".into() => question.title.into(),
            "content".into() => question.content.into(),
            "tags".into() => tags.iter().map(|i| i.as_str()).collect::<Vec<&str>>().into()
        ]
        .into();

        //NOTE: OR

        // let vars = BTreeMap::from([
        //     ("qId".to_string(), question.id.0.into()),
        //     ("title".to_string(), question.title.into()),
        //     ("content".to_string(), question.content.into()),
        //     (
        //         "tags".to_string(),
        //         tags.iter()
        //             .map(|i| i.as_str())
        //             .collect::<Vec<&str>>()
        //             .into(),
        //     ),
        // ]);

        let res = self.execute(sql, Some(vars)).await;
        res
    }

    pub async fn update_question(
        &self,
        question_id: QuestionId,
        question: Question,
    ) -> Result<Vec<Response>, Box<dyn Error>> {
        let sql = "UPDATE questions set qId = $qId, title = $title, content = $content, tags = $tags WHERE qId = $oldId;";

        let tags = question.tags.unwrap_or_default();
        let vars = map![
            "qId".into() => question.id.into(),
            "title".into() => question.title.into(),
            "content".into() => question.content.into(),
            "tags".into() => tags.iter().map(|i| i.as_str()).collect::<Vec<&str>>().into(),
            "oldId".into() => question_id.0.into()
        ]
        .into();

        let res = self.execute(sql, vars).await;
        res
    }

    pub async fn remove_question(
        &self,
        question_id: QuestionId,
    ) -> Result<Vec<Response>, Box<dyn Error>> {
        let sql = "DELETE questions where qId = $qId";
        let vars = map![
            "qId".into() => question_id.into()
        ]
        .into();

        let res = self.execute(sql, vars).await;
        res
    }

    pub async fn get_answer(
        &self,
        question_id: QuestionId,
    ) -> Result<Vec<Response>, Box<dyn Error>> {
        let sql = "SELECT * from answers where qId = $qId;";
        let vars = map![
            "qId".into() => question_id.into(),
        ]
        .into();

        let res = self.execute(sql, vars).await;
        res
    }

    pub async fn add_answer(&self, answer: Answer) -> Result<Vec<Response>, Box<dyn Error>> {
        let sql = "CREATE answers set qId = $qId, aId=$aId, content = $content;";
        let vars = map![
            "qId".into() => answer.question_id.into(),
            "aId".into() => answer.id.0.into(),
            "content".into() => answer.content.into(),
        ]
        .into();

        let res = self.execute(sql, vars).await;
        res
    }

    pub async fn remove_answer(
        &self,
        answer_id: AnswerId,
    ) -> Result<Vec<Response>, Box<dyn Error>> {
        let sql = "SELECT * from answers WHERE aId = $aId;";
        let vars = map![
            "aId".into() => answer_id.0.into(),
        ]
        .into();

        let res = self.execute(sql, vars).await;
        res
    }
}
