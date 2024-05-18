#![feature(proc_macro_hygiene, decl_macro)]

mod api;
mod models;
mod orm;

#[macro_use]
extern crate rocket;
extern crate mongodb;
extern crate serde;

use api::product_api::{status, verify_product};
use orm::mongo_repo::MongoRepo;

use mongodb::{
    options::{ClientOptions, FindOptions},
    Client, Collection, Database,
};

#[launch]
async fn rocket() -> _ {
    let db = MongoRepo::init().await;

    rocket::build()
        .manage(db)
        .mount("/", routes![verify_product, status])
}
