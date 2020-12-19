fn main() {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();

    let settings = settings
        .try_into::<std::collections::HashMap<String, String>>()
        .unwrap();

    let port = settings.get("port").unwrap().parse::<u16>().unwrap();
    let db = settings.get("db").unwrap().as_str();

    product_delivery::rocket(port, db).launch();
}

// #![feature(proc_macro_hygiene, decl_macro)]
// #[macro_use]
// extern crate rocket;
// #[macro_use]
// extern crate diesel;
// #[macro_use]
// extern crate diesel_derive_enum;

// #[macro_use]
// extern crate diesel_migrations;

// extern crate rocket_contrib;

// extern crate config;

// use rocket_cors::{AllowedHeaders, AllowedOrigins};

// use diesel::prelude::*;
// use rocket::config::{Config, Environment, Value};
// use rocket::fairing::AdHoc;
// use rocket_contrib::databases;
// use rocket_contrib::json::Json;
// use std::collections::HashMap;

// pub mod models;
// pub mod schema;

// embed_migrations!("migrations");

// #[databases::database("pg")]
// struct DbConn(databases::diesel::PgConnection);

// fn run_db_migrations(rocket: rocket::Rocket) -> Result<rocket::Rocket, rocket::Rocket> {
//     let conn = DbConn::get_one(&rocket).expect("database connection");
//     match embedded_migrations::run(&*conn) {
//         Ok(()) => Ok(rocket),
//         Err(_) => Err(rocket),
//     }
// }

// fn main() {
//     let allowed_origins = AllowedOrigins::all();

//     let cors = rocket_cors::CorsOptions {
//         allowed_origins,
//         allowed_methods: vec![
//             rocket::http::Method::Get,
//             rocket::http::Method::Post,
//             rocket::http::Method::Put,
//             rocket::http::Method::Delete,
//             rocket::http::Method::Head,
//         ]
//         .into_iter()
//         .map(From::from)
//         .collect(),
//         allowed_headers: AllowedHeaders::all(),
//         allow_credentials: true,
//         ..Default::default()
//     }
//     .to_cors()
//     .unwrap();

//     let mut settings = config::Config::default();
//     settings.merge(config::File::with_name("Settings")).unwrap();

//     let settings = settings.try_into::<HashMap<String, String>>().unwrap();

//     let port = settings.get("port").unwrap().parse::<u16>().unwrap();

//     let mut database_config = HashMap::new();
//     let mut databases = HashMap::new();

//     database_config.insert("url", Value::from(settings.get("db").unwrap().as_str()));
//     databases.insert("pg", Value::from(database_config));

//     let config = Config::build(Environment::Development)
//         .address("0.0.0.0")
//         .port(port)
//         .extra("databases", databases)
//         .finalize()
//         .unwrap();

//     rocket::custom(config)
//         .attach(DbConn::fairing())
//         .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
//         .attach(cors)
//         .mount("/api/", routes![hello, new])
//         .launch();
// }

// #[get("/")]
// fn hello(conn: DbConn) -> Json<Vec<models::Product>> {
//     use schema::products::dsl::*;
//     let our_products = products
//         .load::<models::Product>(&*conn)
//         .expect("Error loading posts");

//     Json(our_products)
// }

// use serde::{Deserialize, Serialize};

// // use hex_literal::hex;
// use sha3::{Digest, Sha3_256};

// use diesel::result::DatabaseErrorKind as DieselErrorKind;
// use diesel::result::Error as DieselError;

// pub mod errors;
// use errors::{ProductDeliveryError, ProductDeliveryResult};

// pub mod endpoints;

// // #[post("/users/register", format = "json", data = "<input>")]
// // fn new(input: Json<RegisterUserRequest>, conn: DbConn) -> ProductDeliveryResult<models::User> {
// //     if input.0.phone > 9_999_999_999 || input.0.phone < 8_000_000_000 {
// //         return Err(ProductDeliveryError::InvalidPhoneNumber);
// //     };

// //     let mut hasher = Sha3_256::new();
// //     hasher.update(input.0.password.as_bytes());

// //     let result = hasher.finalize();
// //     let password_hash = hex::encode(result);

// //     let new_user = models::NewUser {
// //         phone: input.0.phone.to_string(),
// //         role: models::UserRole::Client,
// //         first_name: input.0.first_name,
// //         middle_name: input.0.middle_name,
// //         last_name: input.0.last_name,
// //         password_hash,
// //     };

// //     let user: models::User = match diesel::insert_into(schema::users::table)
// //         .values(&new_user)
// //         .get_result(&*conn)
// //     {
// //         Ok(user) => user,
// //         Err(err) => {
// //             if let DieselError::DatabaseError(DieselErrorKind::UniqueViolation, _) = err {
// //                 return Err(ProductDeliveryError::PhoneAlreadyInUse);
// //             };

// //             return Err(ProductDeliveryError::from(err));
// //         }
// //     };

// //     Ok(Json(user))
// // }
