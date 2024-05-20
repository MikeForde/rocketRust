#[macro_use] extern crate rocket;

use rocket::{Build, Rocket, State};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use mongodb::{Client, Database, options::ClientOptions};
use mongodb::bson::{doc, oid::ObjectId};
use dotenv::dotenv;
use std::env;
use bson::{DateTime};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct IPSModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub packageUUID: String,
    pub patient: Patient,
    pub medication: Vec<Medication>,
    pub allergies: Vec<Allergy>,
    pub conditions: Vec<Condition>,
    #[serde(rename = "__v")]
    pub version: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Patient {
    pub name: String,
    pub given: String,
    pub dob: String,
    pub gender: String,
    pub nation: String,
    pub practitioner: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Medication {
    pub name: String,
    pub date: DateTime,
    pub dosage: String,
    #[serde(rename = "_id")]
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Allergy {
    pub name: String,
    pub criticality: String,
    pub date: DateTime,
    #[serde(rename = "_id")]
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Condition {
    pub name: String,
    pub date: DateTime,
    #[serde(rename = "_id")]
    pub id: ObjectId,
}

struct DbConnection {
    db: Database
}

#[get("/ips/<package_uuid>")]
async fn get_ips(package_uuid: String, db: &State<DbConnection>) -> Option<Json<IPSModel>> {
    // Access the collection
    let collection = db.db.collection::<IPSModel>("ipsalts");

    // Find the document by packageUUID
    let filter = doc! { "packageUUID": package_uuid };
    match collection.find_one(filter, None).await.unwrap() {
        Some(document) => Some(Json(document)),
        None => None,
    }
}

// You normally want to make this a DELETE request, but for simplicity we use GET
#[get("/ips/delbypra/<practitioner>")]
async fn delete_ips_by_practitioner(practitioner: String, db: &State<DbConnection>) -> Json<usize> {
    // Access the collection
    let collection = db.db.collection::<IPSModel>("ipsalts");

    // Delete documents where the practitioner field matches the given value
    let filter = doc! { "patient.practitioner": practitioner };
    let result = collection.delete_many(filter, None).await.unwrap();

    Json(result.deleted_count as usize)
}

#[launch]
async fn rocket() -> Rocket<Build> {
    dotenv().ok();
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI environment variable not set");
    let client_options = ClientOptions::parse(&mongodb_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db: Database = client.database("test");

    rocket::build()
        .manage(DbConnection { db })
        .mount("/", routes![get_ips, delete_ips_by_practitioner])
}
