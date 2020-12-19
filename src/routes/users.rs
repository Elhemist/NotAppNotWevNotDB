use crate::db;
use crate::db::users;
use crate::errors::Error;
use crate::models::user;
use crate::response::ResponseData;
use rocket::http::Status;
use rocket::request::Request;

use rocket::request::FromRequest;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use sha3::{Digest, Sha3_256};

use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct NewUserData {
    phone: i64,
    password: String,
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct LoginData {
    phone: i64,
    password: String,
}

#[post("/users", format = "json", data = "<new_user_data>")]
/// Create new user
pub fn post_users(new_user_data: Json<NewUserData>, conn: db::Conn) -> Result<JsonValue, Error> {
    let new_user_data = new_user_data.into_inner();

    if new_user_data.phone > 9_999_999_999 || new_user_data.phone < 8_000_000_000 {
        return Err(Error::InvalidPhoneNumber);
    };

    let mut hasher = Sha3_256::new();
    hasher.update(new_user_data.password);
    let password_hash = hex::encode(hasher.finalize());

    let new_user = users::NewUser {
        phone: new_user_data.phone.to_string(),
        password_hash,
        role: user::UserRole::Client,
        first_name: new_user_data.first_name,
        middle_name: new_user_data.middle_name,
        last_name: new_user_data.last_name,
    };

    let user = users::create(&conn, new_user)?;

    Ok(json!(ResponseData::success(Some(user))))
}

#[post("/users/login", format = "json", data = "<login_data>")]
pub fn post_users_login(login_data: Json<LoginData>, conn: db::Conn) -> Result<JsonValue, Error> {
    let login_data = login_data.into_inner();

    if login_data.phone > 9_999_999_999 || login_data.phone < 8_000_000_000 {
        return Err(Error::InvalidPhoneNumber);
    };

    let user = users::login(&conn, login_data.phone.to_string(), login_data.password)?;

    Ok(json!(ResponseData::success(Some(user))))
}

// #[derive(Debug)]
// pub struct ApiKey(String);

// impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
//     type Error = crate::errors::Error;

//     fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<Self, Self::Error> {
//         let keys: Vec<_> = request.headers().get("x-session-id").collect();
//         rocket::request::Outcome::Failure((Status::BadRequest, Error::InvalidSessionID))
//     }
// }

// #[post("/users/logout")]
// pub fn post_users_logout(conn: db::Conn, key: ApiKey) -> Result<JsonValue, Error> {
//     println!("{:?}", key);

//     Ok(json!(()))
// }
