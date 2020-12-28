use crate::response;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{status, Responder};
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Error {
    DatabaseError,
    InvalidPhoneNumber,
    PhoneAlreadyInUse,
    UserNotFound,
    InvalidPassword,
    InvalidSessionId,
    NotFound,
    CartIsEmpty,
    AccessDenied,
    Empty,
}

impl Error {
    pub fn status(&self) -> Status {
        match self {
            Error::InvalidPhoneNumber
            | Error::PhoneAlreadyInUse
            | Error::UserNotFound
            | Error::InvalidPassword => Status::BadRequest,
            Error::CartIsEmpty => Status::BadRequest,
            Error::InvalidSessionId => Status::Unauthorized,
            Error::AccessDenied => Status::Forbidden,
            Error::NotFound => Status::NotFound,
            Error::Empty => Status::NoContent,
            _ => Status::BadGateway,
        }
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, req: &Request) -> rocket::response::Result<'r> {
        status::Custom(
            self.status(),
            Json(json!(response::ResponseData::<()> {
                error: true,
                code: json!(self),
                data: None,
            })),
        )
        .respond_to(req)
    }
}

impl From<DieselError> for Error {
    fn from(e: DieselError) -> Error {
        match e {
            DieselError::NotFound => Error::NotFound,
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, info) => {
                if info.constraint_name() == Some("unique_phone") {
                    Error::PhoneAlreadyInUse
                } else {
                    Error::DatabaseError
                }
            }
            _ => Error::DatabaseError,
        }
    }
}
