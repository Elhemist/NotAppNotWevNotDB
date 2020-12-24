use crate::db;
use crate::db::users;
use crate::errors::Error;
use crate::models::user;
use crate::response::ResponseData;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::request::{Outcome, Request};

use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use sha3::{Digest, Sha3_256};
use user::UserRole;

use super::AuthorizedUser;

#[derive(Deserialize)]
pub struct NewUserData {
    phone: i64,
    password: String,
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginData {
    phone: i64,
    password: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for super::AuthorizedUser {
    type Error = crate::errors::Error;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let conn = match db::Conn::from_request(request) {
            Outcome::Success(conn) => conn,
            _ => {
                return Outcome::Failure((Status::Unauthorized, Error::InvalidSessionId));
            }
        };

        let sessions: Vec<_> = request.headers().get("x-session-id").collect();
        if sessions.len() != 1 {
            return Outcome::Failure((Status::Unauthorized, Error::InvalidSessionId));
        }

        let user = match sessions.first() {
            Some(session_id) => match users::user_by_session_id(&conn, session_id.to_string()) {
                Ok(user) => user,
                Err(e) => {
                    return Outcome::Failure((Status::Unauthorized, e));
                }
            },
            _ => return Outcome::Failure((Status::Unauthorized, Error::InvalidSessionId)),
        };

        Outcome::Success(super::AuthorizedUser(user))
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for super::AuthorizedAdmin {
    type Error = crate::errors::Error;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let user = AuthorizedUser::from_request(&request)?.0;

        let user = match user.role {
            UserRole::Admin => user,
            _ => return Outcome::Failure((Status::Forbidden, Error::AccessDenied)),
        };

        Outcome::Success(super::AuthorizedAdmin(user))
    }
}

#[get("/users/current")]
pub fn get_current_user(
    authorized_user: Result<super::AuthorizedUser, Error>,
) -> Result<JsonValue, Error> {
    let mut authorized_user = authorized_user?;
    authorized_user.0.session_id = None;

    Ok(json!(ResponseData::success(Some(authorized_user.0))))
}

#[get("/users/<id>")]
pub fn get_user_by_id(conn: db::Conn, id: i32) -> Result<JsonValue, Error> {
    let mut user = users::user_by_id(&conn, id)?;
    user.session_id = None;

    Ok(json!(ResponseData::success(Some(user))))
}

#[post("/users", format = "json", data = "<new_user_data>")]
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

#[post("/users/logout")]
pub fn post_users_logout(
    conn: db::Conn,
    authorized_user: Result<super::AuthorizedUser, Error>,
) -> Result<JsonValue, Error> {
    let authorized_user = authorized_user?;

    users::logout(&conn, authorized_user.0.id)?;

    Ok(json!(ResponseData::success(Some(()))))
}
