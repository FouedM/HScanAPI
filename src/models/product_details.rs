use std::clone;

use rocket::form::FromForm;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductDetails {
    pub product_name: String,
    pub ingredients_text: Option<String>,
    pub brands: Option<String>,
    //  categories: String,
    //   packaging: String,
    pub image_url: Option<String>,
    //   additives_n: i32,
    pub additives_tags: Option<Vec<String>>,
    // allergens: String,
    pub traces: Option<String>,
    // stores: String,
    pub manufacturing_places: Option<String>,
    pub labels: Option<String>,
}

#[derive(Debug, FromForm, Serialize, Deserialize, Clone)]
pub struct Product {
    pub barCode: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FoodFact {
    pub product: Option<ProductDetails>,
    pub code: String,
    pub status: i32,
    pub status_verbose: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub food_fact: FoodFact,
    pub decision: Decision,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Decision {
    pub status: String,
    pub description: String,
}
