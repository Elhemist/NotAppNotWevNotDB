#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;

extern crate config;

extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use rocket::config::{Config, Environment, Value};
use rocket_contrib::databases;
use rocket_contrib::json::Json;
use std::collections::HashMap;
use std::env;

pub mod models;
pub mod schema;
#[databases::database("pg")]
struct DbConn(databases::diesel::PgConnection);

fn main() {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();

    let settings = settings.try_into::<HashMap<String, String>>().unwrap();

    let port = settings.get("port").unwrap().parse::<u16>().unwrap();

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    database_config.insert("url", Value::from(settings.get("db").unwrap().as_str()));
    databases.insert("pg", Value::from(database_config));

    let config = Config::build(Environment::Development)
        .address("0.0.0.0")
        .port(port)
        .extra("databases", databases)
        .finalize()
        .unwrap();

    rocket::custom(config).mount("/", routes![hello]).launch();
}

#[get("/")]
fn hello(conn: DbConn) -> Json<Vec<models::Product>> {
    use schema::products::dsl::*;
    let our_products = products
        .load::<models::Product>(&*conn)
        .expect("Error loading posts");

    Json(our_products)
}
