use crate::db::{self, cart};
use crate::errors::Error;
use crate::response::ResponseData;

use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ProductInCart {
    pub product_id: i32,
    pub quantity: i32,
}

#[get("/cart")]
pub fn get(
    conn: db::Conn,
    authorized_user: Result<super::AuthorizedUser, Error>,
) -> Result<JsonValue, Error> {
    let authorized_user = authorized_user?;

    let products = match cart::get(&conn, &authorized_user.0) {
        Ok(products) => products,
        Err(e) => return Err(e),
    }
    .iter()
    .map(|product| ProductInCart {
        product_id: product.0,
        quantity: product.1,
    })
    .collect::<Vec<_>>();

    Ok(json!(ResponseData::success(Some(products))))
}

#[post("/cart", format = "json", data = "<update>")]
pub fn update(
    conn: db::Conn,
    authorized_user: Result<super::AuthorizedUser, Error>,
    update: Json<ProductInCart>,
) -> Result<JsonValue, Error> {
    let authorized_user = authorized_user?.0;

    let update = update.into_inner();

    if update.quantity > 0 {
        cart::add(&conn, &authorized_user, update.product_id, update.quantity)
    } else {
        cart::remove(&conn, &authorized_user, update.product_id)
    }?;

    Ok(json!(ResponseData::success(Some(()))))
}
