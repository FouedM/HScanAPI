use std::env;
use dotenv::dotenv;
use mongodb::{
    bson::{doc, Document, from_document, oid::ObjectId, Bson},
    options::{ClientOptions},
    Client,
    Collection,
};
use mongodb::error::Error;
use crate::models::code::Code;
use futures::stream::StreamExt;

pub struct MongoRepo {
    col: Collection<Code>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let client_options = ClientOptions::parse(&database_url).unwrap();
        let client = Client::with_options(client_options).unwrap();
        let db = client.database("halalScan");
        let col: Collection<Code> = db.collection("codes");
        MongoRepo { col }
    }

    pub async fn get_all_codes(&self, search:Vec<String>) -> Result<Vec<Code>, Error> {
        println!("{:?}", search);

        let filter = doc! {
            "number": {
                "$in": search
            }
        };


        let mut cursors = self
            .col
            .find(filter, None)
            .await?;
            //.ok()
            //.expect("Error getting codes");

            let mut codes: Vec<Code> = Vec::new();

            while let Some(doc) = cursors.next().await {
               // println!("test here");
             //   println!("{:?}", doc);
             codes.push(doc?);
            }
        
            Ok(codes)
            // while let Some(doc) = cursors.next().await {
            //     println!("{:?}", doc);
            //    // codes.push(doc?);
            // }
           // Ok(codes)
    }
}