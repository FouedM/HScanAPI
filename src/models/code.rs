use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    _id: ObjectId,
    number: String,
    name: String,
    description: String,
    halalStatus: String,
    pub decision: i32,
}