use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId};

#[derive(Serialize, Deserialize, Debug)]
pub struct QuizRequest {
    pub subject: String,
    pub content: QuizContent,
    pub keywords: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuizYamlRequest {
    pub yaml: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quiz {
    pub _id: Option<ObjectId>,
    pub subject: String,
    pub content: QuizContent,
    pub keywords: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuizContent {
    pub text: String,
    pub sub_text: Option<String>,
    pub answers: Vec<String>,
    pub correct_indexes: Vec<i32>,
    pub examination: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuizzesRequest {
    pub limit: Option<i64>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct RecentAddedQuizzesRequest {
    pub limit: Option<i64>
}