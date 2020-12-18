#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;

#[macro_use]
extern crate diesel_migrations;

extern crate rocket_contrib;

extern crate config;

extern crate dotenv;

use rocket_cors::{AllowedHeaders, AllowedOrigins};

use diesel::prelude::*;
use rocket::config::{Config, Environment, Value};
use rocket::fairing::AdHoc;
use rocket_contrib::databases;
use rocket_contrib::json::Json;
use std::collections::HashMap;

pub mod models;
pub mod schema;

embed_migrations!("migrations");

#[databases::database("pg")]
struct DbConn(databases::diesel::PgConnection);

fn run_db_migrations(rocket: rocket::Rocket) -> Result<rocket::Rocket, rocket::Rocket> {
    let conn = DbConn::get_one(&rocket).expect("database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(_) => Err(rocket),
    }
}

fn main() {
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![
            rocket::http::Method::Get,
            rocket::http::Method::Post,
            rocket::http::Method::Put,
            rocket::http::Method::Delete,
            rocket::http::Method::Head,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

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

    rocket::custom(config)
        .attach(DbConn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .attach(cors)
        .mount("/api/", routes![hello, new])
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

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct RegisterUserRequest {
    phone: i64,
    password: String,
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
}

// use hex_literal::hex;
use sha3::{Digest, Sha3_256};

use rocket::http::Status;
use rocket::response::status;

#[post("/users/register", format = "json", data = "<input>")]
fn new(input: Json<RegisterUserRequest>, conn: DbConn) -> ApiResult<models::User> {
    if input.0.phone > 9_999_999_999 || input.0.phone < 8_000_000_000 {
        return Err(ApiError::InvalidPhoneNumber);
    };

    let mut hasher = Sha3_256::new();
    hasher.update(input.0.password.as_bytes());

    let result = hasher.finalize();
    let password_hash = hex::encode(result);

    let new_user = models::NewUser {
        phone: input.0.phone.to_string(),
        role: models::UserRole::Client,
        first_name: input.0.first_name,
        middle_name: input.0.middle_name,
        last_name: input.0.last_name,
        password_hash,
    };

    let user: models::User = match diesel::insert_into(schema::users::table)
        .values(&new_user)
        .get_result(&*conn)
    {
        Ok(user) => user,
        Err(err) => {
            if let DieselError::DatabaseError(DieselErrorKind::UniqueViolation, _) = err {
                return Err(ApiError::PhoneAlreadyInUse);
            };

            return Err(ApiError::from(err));
        }
    };

    Ok(Json(user))
}

use diesel::result::DatabaseErrorKind as DieselErrorKind;
use diesel::result::Error as DieselError;
use rocket::response::{Responder, Response};

#[derive(Debug)]
pub enum ApiError {
    DieselError,
    UnknownError,
    InvalidPhoneNumber,
    PhoneAlreadyInUse,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: bool,
    pub code: String,
    pub description: String,
}

pub type ApiResult<T> = Result<Json<T>, ApiError>;

impl<'r> Responder<'r> for ApiError {
    fn respond_to(self, _: &rocket::Request) -> rocket::response::Result<'r> {
        let (code, description) = match self {
            ApiError::DieselError => (
                String::from("DATABASE_ERROR"),
                String::from("Unknown error with database"),
            ),
            ApiError::InvalidPhoneNumber => (
                String::from("INVALID_PHONE_NUMBER"),
                String::from("Invalid phone number"),
            ),
            ApiError::UnknownError => (
                String::from("UNKNOWN_ERROR"),
                String::from("Unknown error. ¯\\_(ツ)_/¯"),
            ),
            ApiError::PhoneAlreadyInUse => (
                String::from("PHONE_ALREADY_IN_USE"),
                String::from("Phone already in use"),
            ),
        };

        let err = ErrorResponse {
            error: true,
            code,
            description,
        };

        let json_string = match serde_json::to_string(&err) {
            Ok(json_string) => json_string,
            Err(_) => return Response::build().status(Status::BadGateway).ok(),
        };

        Response::build()
            .status(Status::BadRequest)
            .sized_body(std::io::Cursor::new(json_string))
            .ok()
    }
}

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> ApiError {
        ApiError::DieselError
    }
}
