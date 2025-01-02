extern crate mongodb;
use std::io::Error;

use mongodb::bson::doc;
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
}
async fn init() {
    let client_options = ClientOptions::parse("mongondb://localhost:27017")
        .await
        .unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db_name = "mydatabase";
    let coll_name = "mycollection";
    create_collection(&client, db_name, coll_name).await;
}
async fn main() {
    init().await;
}
async fn create_collection(client: &Client, db_name: &str, coll_name: &str) {
    let db = client.database(db_name);
    db.create_collection(db_name).await.unwrap();
    insert_document(client, db_name, coll_name).await;
}

async fn insert_document(client: &Client, db_name: &str, coll_name: &str) {
    let db = client.database(db_name);
    let coll = db.collection(coll_name);
    let doc = doc! { "name": "John", "age": 30  };
    coll.insert_one(doc).await.unwrap();
}

async fn get_document(client: &Client, db_name: &str, coll_name: &str) {
    let db = client.database(db_name);
    let coll = db.collection::<Person>(coll_name);
    let filter = doc! { "name": "John" };
    let result = coll.find_one(filter).await.unwrap();
    match result {
        Some(doc) => println!("{:?}", doc),
        None => println!("No document found"),
    }
}

async fn delete_document(client: &Client, db_name: &str, coll_name: &str) {
    let db = client.database(db_name);
    let coll = db.collection::<Person>(coll_name);
    let filter = doc! { "name": "John"};
    coll.delete_one(filter).await.unwrap();
}

async fn update_document(client: &Client, db_name: &str, coll_name: &str) {
    let db = client.database(db_name);
    let coll = db.collection::<Person>(coll_name);
    let filter = doc! { "name": "John"};
    let update = doc! {"$set": {"age": 31}};
    coll.update_one(filter, update).await.unwrap();
}
