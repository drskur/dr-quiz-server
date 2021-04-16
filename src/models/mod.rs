use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId};

#[derive(Serialize, Deserialize, Debug)]
pub struct Quiz {
    pub _id: ObjectId,
    pub subject: String
}

#[derive(Serialize, Deserialize)]
pub struct AddQuizRequest {
    pub subject: String,
}
