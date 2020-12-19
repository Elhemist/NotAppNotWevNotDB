use crate::response;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{status, Responder};
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Error {
    Unknown,
    DatabaseError,
    InvalidPhoneNumber,
    PhoneAlreadyInUse,
    UserNotFound,
    InvalidPassword,
    InvalidSessionID,
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, req: &Request) -> rocket::response::Result<'r> {
        status::Custom(
            Status::BadGateway,
            Json(json!(response::ResponseData::<()> {
                error: true,
                code: json!(self),
                data: None,
            })),
        )
        .respond_to(req)
    }
}

// impl From<dyn std::error::Error> for Error {
//     fn from(_: std::error::Error) -> Self {
//         todo!()
//     }
// }

// use diesel::result::Error as DieselError;
// use rocket::http::Status;
// use rocket::response::{Responder, Response};
// use rocket_contrib::json::Json;
// use serde::Serialize;

// #[derive(Debug, Serialize)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// pub enum ProductDeliveryError {
//     UnknownError,
//     DatabaseError,
//     InvalidPhoneNumber,
//     PhoneAlreadyInUse,
// }

// #[derive(Serialize)]
// pub struct ErrorResponse {
//     pub error: bool,
//     pub code: String,
//     pub description: String,
// }

// pub type ProductDeliveryResult<T> = Result<Json<T>, ProductDeliveryError>;

// impl<'r> Responder<'r> for ProductDeliveryError {
//     fn respond_to(self, _: &rocket::Request) -> rocket::response::Result<'r> {
//         let (code, description) = match self {
//             ProductDeliveryError::DieselError => (
//                 String::from("DATABASE_ERROR"),
//                 String::from("Unknown error with database"),
//             ),
//             ProductDeliveryError::InvalidPhoneNumber => (
//                 String::from("INVALID_PHONE_NUMBER"),
//                 String::from("Invalid phone number"),
//             ),
//             ProductDeliveryError::UnknownError => (
//                 String::from("UNKNOWN_ERROR"),
//                 String::from("Unknown error. ¯\\_(ツ)_/¯"),
//             ),
//             ProductDeliveryError::PhoneAlreadyInUse => (
//                 String::from("PHONE_ALREADY_IN_USE"),
//                 String::from("Phone already in use"),
//             ),
//         };

//         let err = ErrorResponse {
//             error: true,
//             code,
//             description,
//         };

//         let json_string = match serde_json::to_string(&err) {
//             Ok(json_string) => json_string,
//             Err(_) => return Response::build().status(Status::BadGateway).ok(),
//         };

//         Response::build()
//             .status(Status::BadRequest)
//             .sized_body(std::io::Cursor::new(json_string))
//             .ok()
//     }
// }

// impl From<DieselError> for ProductDeliveryError {
//     fn from(error: DieselError) -> ProductDeliveryError {
//         ProductDeliveryError::DieselError
//     }
// }
