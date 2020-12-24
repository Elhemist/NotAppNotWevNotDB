use crate::db::{self, orders};
use crate::errors::Error;
use crate::response::ResponseData;

use rocket_contrib::json::JsonValue;

#[post("/orders")]
pub fn create(
    conn: db::Conn,
    authorized_user: Result<super::AuthorizedUser, Error>,
) -> Result<JsonValue, Error> {
    let authorized_user = authorized_user?.0;

    let order_info = orders::create(&conn, &authorized_user)?;

    Ok(json!(ResponseData::success(Some(order_info))))
}

#[get("/orders")]
pub fn list(
    conn: db::Conn,
    authorized_user: Result<super::AuthorizedUser, Error>,
) -> Result<JsonValue, Error> {
    let authorized_user = authorized_user?.0;

    let orders_info = orders::history(&conn, &authorized_user)?;

    Ok(json!(ResponseData::success(Some(orders_info))))
}

#[get("/orders/<id>")]
pub fn get(conn: db::Conn, id: i32) -> Result<JsonValue, Error> {
    let order_info = orders::get(&conn, id)?;

    Ok(json!(ResponseData::success(Some(order_info))))
}
