mod api;
mod models;
mod orm;

#[macro_use]
extern crate rocket;
extern crate mongodb;
extern crate serde;

use api::history::{add_history, get_history, init_user};
use api::product_api::{status, verify_product};
use orm::mongo_repo::MongoRepo;

#[launch]
async fn rocket() -> _ {
    let db = MongoRepo::init().await;

    rocket::build()
        .manage(db)
        .mount("/", routes![verify_product])
        .mount("/", routes![get_history])
        .mount("/", routes![add_history])
        .mount("/", routes![init_user])
        .mount("/", routes![status])
}
