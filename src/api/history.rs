use crate::models::history::{AddHistoryForm, History};
use crate::models::scan::Scan;
use crate::{
    models::product_details::{FoodFact, Product, ProductDetails, Response},
    orm::mongo_repo::MongoRepo,
};
use mongodb::bson::raw::ErrorKind;
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use reqwest::Error;
use rocket::form::{Form, FromForm};
use rocket::{http::Status, serde::json::Json, State};

#[post("/add_history", data = "<form>")]
pub async fn add_history(
    form: Form<AddHistoryForm>,
    db: &State<MongoRepo>,
) -> Result<&'static str, Status> {
    db.add_history(&form.u_id, &form.scan_id).await;
    Ok("History added")
}

#[get("/get_history/<u_id>")]
pub async fn get_history(
    db: &State<MongoRepo>,
    u_id: String,
) -> Result<Option<Json<Response>>, Status> {
    println!("{:?}", u_id);
    if u_id.is_empty() {
        return Err(Status::BadRequest);
    };
    let history_data = db.get_history(&u_id).await;
    let mut result = {};
    Ok(None)
}

#[get("/init_user")]
pub async fn init_user(db: &State<MongoRepo>) -> Result<Json<String>, Status> {
    let non_exist_id = ObjectId::new().to_string();
    Ok(Json(non_exist_id))
}
