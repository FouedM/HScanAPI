use crate::models::code::Code;
use crate::models::product_details::Decision;
use crate::{
    models::product_details::{FoodFact, Product, ProductDetails, Response},
    orm::mongo_repo::MongoRepo,
};
use dotenv::dotenv;
use mongodb::{
    bson::oid::ObjectId,
    options::{ClientOptions, FindOptions},
    results::InsertOneResult,
};
use reqwest::Error;
use rocket::form::{Form, FromForm};
use rocket::{http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use std::env;

fn check_codes(codes: Vec<Code>) -> Decision {
    let mut response = Decision {
        status: "halal".to_string(),
        description: "".to_string(),
    };
    let mut has_one = false;
    for code in codes {
        match code.decision {
            2 => {
                response = Decision {
                    status: "haram".to_string(),
                    description: "This product contains haram ingredients".to_string(),
                }
            }
            1 => has_one = true,
            _ => (),
        }
    }
    if has_one {
        Decision {
            status: "doubt".to_string(),
            description: "This product may contain haram ingredients".to_string(),
        }
    } else {
        response
    }
}

#[get("/status")]
pub fn status() -> &'static str {
    "API is up and running"
}

#[post("/verifyProduct", data = "<product>")]
pub async fn verify_product(
    db: &State<MongoRepo>,
    product: Json<Product>,
) -> Result<Json<Response>, Status> {
    dotenv().ok();
    let base_url = env::var("OPEN_FOOD_FACTS_API_URL").unwrap_or_default();
    println!("{:?}", product);
    let url = format!("{}/{}.json", base_url, product.barCode);
    let response = reqwest::get(&url).await;

    let response_result: Result<FoodFact, Error> = match response {
        Ok(resp) => {
            // println!("{:?}", resp);
            match resp.json::<FoodFact>().await {
                Ok(food_fact) => Ok(food_fact),
                Err(_) => return Err(Status::InternalServerError),
            }
        }
        Err(_) => return Err(Status::InternalServerError),
    };
    println!("response_result {:?}", response_result);

    match response_result {
        Ok(food_fact) => {
            println!("{:?}", food_fact);

            let tags = food_fact
                .product
                .additives_tags
                .clone()
                .unwrap_or_else(Vec::new);

            let mut decision = Decision {
                status: "Unkowen".to_string(),
                description: "Ingredients are missing".to_string(),
            };

            if food_fact.product.ingredients_text.is_none() {
                return Ok(Json(Response {
                    food_fact: food_fact,
                    decision: decision,
                }));
            }

            if food_fact.product.ingredients_text.is_some() && tags.is_empty() {
                return Ok(Json(Response {
                    food_fact: food_fact,
                    decision: Decision {
                        status: "halal".to_string(),
                        description: "This product does not contain any additives".to_string(),
                    },
                }));
            }

            if !tags.is_empty() {
                let modified_traces: Vec<_> = tags
                    .clone()
                    .into_iter()
                    .map(|s| {
                        s.splitn(2, ":")
                            .nth(1)
                            .unwrap_or(&s)
                            .to_string()
                            .to_uppercase()
                    })
                    .collect();
                let codes = db.get_all_codes(modified_traces).await;

                decision = check_codes(codes.unwrap());
                println!("{:?}", decision);
            }

            let mut result: Response = Response {
                food_fact: food_fact.clone(),
                decision: decision,
            };
            Ok(Json(result))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
