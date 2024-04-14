#![feature(proc_macro_hygiene, decl_macro)]

mod api;
mod models;
mod orm;

#[macro_use]
extern crate rocket;
extern crate mongodb;
extern crate serde;

use api::history::{add_history, get_history, init_user};
use api::product_api::verify_product;
use orm::mongo_repo::MongoRepo;

use dotenv::dotenv;
use mongodb::bson::{doc, Document};
use mongodb::{
    options::{ClientOptions, FindOptions},
    Client, Collection, Database,
};
use reqwest::Error;
use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use std::env;

#[launch]
async fn rocket() -> _ {
    let db = MongoRepo::init().await;

    rocket::build()
        .manage(db)
        .mount("/", routes![verify_product])
        .mount("/", routes![get_history])
        .mount("/", routes![add_history])
        .mount("/", routes![init_user])
}
