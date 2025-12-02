#[macro_use]
extern crate rocket;

mod db;
mod models;
mod routes;

use dotenv::dotenv;
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;
use sqlx::mysql::MySqlPoolOptions;
use std::env;

use db::MySqlPool;
use routes::api::{delete_ips_by_practitioner, get_ips};
use routes::pages::{index, ips_view};

#[launch]
async fn rocket() -> Rocket<Build> {
    dotenv().ok();

    let db_host = env::var("DB_HOST").expect("DB_HOST environment variable not set");
    let db_name = env::var("DB_NAME").expect("DB_NAME environment variable not set");
    let db_user = env::var("DB_USER").expect("DB_USER environment variable not set");
    let db_pass = env::var("DB_PASSWORD").expect("DB_PASSWORD environment variable not set");

    // e.g. mysql://user:pass@localhost/ipsdb
    let database_url = format!("mysql://{}:{}@{}/{}", db_user, db_pass, db_host, db_name);

    let pool: MySqlPool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create MySQL pool");

    rocket::build()
        .manage(pool)
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                index,
                ips_view,
                get_ips,
                delete_ips_by_practitioner
            ],
        )
}
