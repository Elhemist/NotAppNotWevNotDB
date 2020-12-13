#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;

extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use rocket_contrib::databases;
use rocket_contrib::json::Json;
use std::env;

pub mod models;
pub mod schema;
#[databases::database("pg")]
struct DbConn(databases::diesel::PgConnection);

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![hello])
        .attach(DbConn::fairing())
        .launch();
}

#[get("/")]
fn hello(conn: DbConn) -> Json<Vec<models::Product>> {
    use schema::products::dsl::*;
    let our_products = products
        .load::<models::Product>(&*conn)
        .expect("Error loading posts");

    Json(our_products)
}
