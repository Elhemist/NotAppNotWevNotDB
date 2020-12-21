use crate::db::{self, products};
use crate::errors::Error;
use crate::response::ResponseData;

use rocket_contrib::json::JsonValue;

#[get("/products")]
pub fn get_products(conn: db::Conn) -> Result<JsonValue, Error> {
    let all_products = products::get_all(&conn)?;

    Ok(json!(ResponseData::success(Some(all_products))))
}
