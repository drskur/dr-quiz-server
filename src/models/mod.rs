use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId};

#[derive(Serialize, Deserialize, Debug)]
pub struct Quiz {
    pub _id: Option<ObjectId>,
    pub subject: String,
    pub body: String,
}