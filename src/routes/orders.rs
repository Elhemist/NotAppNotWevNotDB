use crate::db::{self, orders};
use crate::errors::Error;
use crate::response::ResponseData;

use rocket_contrib::json::{Json, JsonValue};

#[get("/orders")]
pub fn get(
    conn: db::Conn,
    authorized_user: Result<super::AuthorizedUser, Error>,
) -> Result<JsonValue, Error> {
    let authorized_user = authorized_user?;

    let order = match orders::create(&conn, &authorized_user.0) {
        Ok(order) => order,
        Err(e) => return Err(e),
    };

    Ok(json!(ResponseData::success(Some(order))))
}
