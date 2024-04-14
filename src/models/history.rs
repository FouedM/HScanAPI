use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    pub u_id: ObjectId,
    pub scan_id: ObjectId,
}

#[derive(FromForm)]
pub struct AddHistoryForm {
    pub scan_id: String,
    pub u_id: String,
}

pub struct History_response {}
