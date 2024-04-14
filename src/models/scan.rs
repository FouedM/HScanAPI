use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::product_details::{Decision, FoodFact, Product};

#[derive(Debug, Serialize, Deserialize)]
pub struct Scan {
    pub food_fact: FoodFact,
    pub decision: Decision,
}
