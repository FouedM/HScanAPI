use crate::api::history;
use crate::models::history::History;
use crate::models::product_details::Product;
use crate::models::{code::Code, scan::Scan};
use dotenv::dotenv;
use futures::stream::StreamExt;
use mongodb::bson::{to_bson, to_document};
use mongodb::error::Error;
use mongodb::results::InsertOneResult;
use mongodb::{
    bson::{doc, from_document, oid::ObjectId, Bson, Document},
    options::ClientOptions,
    Client, Collection,
};
use std::env;

pub struct MongoRepo {
    code: Collection<Code>,
    scan: Collection<Scan>,
    history: Collection<History>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let client_options = ClientOptions::parse(&database_url).unwrap();
        let client = Client::with_options(client_options).unwrap();
        let db = client.database("halalScan");
        let code: Collection<Code> = db.collection("codes");
        let scan: Collection<Scan> = db.collection("scan");
        let history: Collection<History> = db.collection("history");
        MongoRepo {
            code,
            scan,
            history,
        }
    }

    pub async fn get_all_codes(&self, search: Vec<String>) -> Result<Vec<Code>, Error> {
        println!("{:?}", search);

        let filter = doc! {
            "number": {
                "$in": search
            }
        };

        let mut cursors = self.code.find(filter, None).await?;

        let mut codes: Vec<Code> = Vec::new();

        while let Some(doc) = cursors.next().await {
            codes.push(doc?);
        }

        Ok(codes)
    }

    pub async fn find_scan(&self, product: Product) -> Result<Scan, Error> {
        let filter = doc! {
            "product": {
                "$eq": product.barCode
            }
        };
        let scan = self.scan.find_one(filter, None).await?;

        while let Some(doc) = scan {
            return Ok(doc);
        }

        Ok(scan.unwrap())
    }

    pub async fn add_scan(&self, scan: Scan) -> Result<InsertOneResult, Error> {
        println!("add_scan");
        let scan_doc = Scan {
            food_fact: scan.food_fact,
            decision: scan.decision,
        };
        let new_scan = self.scan.insert_one(scan_doc, None).await.unwrap();
        println!("{:?}", new_scan);
        Ok(new_scan)
    }

    pub async fn add_history(
        &self,
        u_id: &String,
        scand_id: &String,
    ) -> Result<InsertOneResult, Error> {
        let new_u_id = ObjectId::parse_str(&u_id).unwrap();
        let new_scan_id = ObjectId::parse_str(&scand_id).unwrap();
        let history_doc = History {
            u_id: new_u_id,
            scan_id: new_scan_id,
        };

        let new_history = self.history.insert_one(history_doc, None).await.unwrap();
        Ok(new_history)
    }

    pub async fn get_history(&self, u_id: &String) -> Result<Vec<Scan>, Error> {
        let mut user_scans: Vec<Scan> = Vec::new();

        if !u_id.is_empty() {
            let mut filter = doc! {};

            let obj_id = ObjectId::parse_str(&u_id).unwrap();
            filter = doc! {
                "u_id": obj_id
            };

            let pipeline = vec![
                doc! { "$match": filter },
                doc! { "$lookup": {
                    "from": "scan",
                    "localField": "scan_id",
                    "foreignField": "_id",
                    "as": "scan"
                }},
                doc! { "$unwind": "$scan" },
            ];

            println!("{:?}", pipeline);

            let mut user_history = match self.history.aggregate(pipeline, None).await {
                Ok(cursor) => cursor,
                Err(e) => {
                    eprintln!("Error executing aggregation: {}", e);
                    return Err(e);
                }
            };
            //println!("{:?}", user_history);

            while let Some(result) = user_history.next().await {
                print!("{:?}", result);
                match result {
                    Ok(document) => {
                        println!("{:?}", document);
                        // `document` is one of the documents returned by the aggregation operation.
                        // You can access its fields like this:
                        let scan_id = document.get("scan_id").and_then(Bson::as_str);
                        println!("scan_id: {:?}", scan_id);
                        // Do something with `field_value`...
                    }
                    Err(e) => {
                        // Handle the error...
                        println!("{:?}", e);
                    }
                }
            }

            //  let mut scans = self.scan.find(filter, None).await?;

            // let mut user_history: Vec<Scan> = Vec::new();

            // while let Some(doc) = scans.next().await {
            //     // user_history.push(doc?);
            //     match doc {
            //         Ok(document) => {
            //             let u_history: Scan = mongodb::bson::from_document(scan).unwrap();
            //             user_history.push(u_history);
            //         }
            //         Err(e) => return Err(e.into()),
            //     }
            // }
        }
        return Ok(user_scans);
    }
}
