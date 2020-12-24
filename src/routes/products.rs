use crate::db::{self, products};
use crate::errors::Error;
use crate::response::ResponseData;

use products::NewProduct;
use rocket_contrib::json::{Json, JsonValue};

#[get("/products")]
pub fn get_products(conn: db::Conn) -> Result<JsonValue, Error> {
    let all_products = products::get_all(&conn)?;

    Ok(json!(ResponseData::success(Some(all_products))))
}

#[get("/products/<id>")]
pub fn get_product_by_id(conn: db::Conn, id: i32) -> Result<JsonValue, Error> {
    let product = products::get(&conn, id)?;

    Ok(json!(ResponseData::success(Some(product))))
}

#[post("/products", format = "json", data = "<new_product_data>")]
pub fn post_products(
    authorized_admin: Result<super::AuthorizedAdmin, Error>,
    new_product_data: Json<NewProduct>,
    conn: db::Conn,
) -> Result<JsonValue, Error> {
    println!("{:?}", authorized_admin);
    let _ = authorized_admin?.0;

    let new_product = products::create(&conn, new_product_data.0)?;

    Ok(json!(ResponseData::success(Some(new_product))))
}
