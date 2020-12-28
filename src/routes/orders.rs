use crate::errors::Error;
use crate::response::ResponseData;
use crate::{
    db::{self, orders},
    models::order::NewAddress,
};
use serde::Deserialize;

use rocket_contrib::json::{Json, JsonValue};

#[derive(Deserialize, Debug)]
pub struct NewOrderData {
    pub street: String,
    pub home: String,
    pub apartment: Option<String>,
    pub comment: Option<String>,
}

#[post("/orders", format = "json", data = "<new_order_data>")]
pub fn create(
    conn: db::Conn,
    new_order_data: Json<NewOrderData>,
    authorized_user: Result<super::AuthorizedUser, Error>,
) -> Result<JsonValue, Error> {
    let authorized_user = authorized_user?.0;
    let new_order_data = new_order_data.into_inner();

    let order_info = orders::create(
        &conn,
        &authorized_user,
        NewAddress {
            user_id: authorized_user.id,
            street: new_order_data.street,
            home: new_order_data.home,
            apartment: new_order_data.apartment,
        },
        new_order_data.comment,
    )?;

    Ok(json!(ResponseData::success(Some(order_info))))
}

#[get("/orders")]
pub fn list(
    conn: db::Conn,
    authorized_user: Result<super::AuthorizedUser, Error>,
) -> Result<JsonValue, Error> {
    let authorized_user = authorized_user?.0;

    let orders_info = match authorized_user.role {
        crate::models::user::UserRole::Client => orders::history(&conn, &authorized_user),
        crate::models::user::UserRole::Admin => orders::global_history(&conn),
        crate::models::user::UserRole::Courier => {
            orders::list_orders_for_courier(&conn, &authorized_user)
        }
    }?;

    Ok(json!(ResponseData::success(Some(orders_info))))
}

#[get("/orders/<id>")]
pub fn get(conn: db::Conn, id: i32) -> Result<JsonValue, Error> {
    let order_info = orders::get(&conn, id)?;

    Ok(json!(ResponseData::success(Some(order_info))))
}

#[post("/orders/pick")]
pub fn pick(
    conn: db::Conn,
    authorized_courier: Result<super::AuthorizedCourier, Error>,
) -> Result<JsonValue, Error> {
    let authorized_courier = authorized_courier?.0;

    let order_info = orders::pick_order_for_courier(&conn, &authorized_courier)?;

    Ok(json!(ResponseData::success(Some(order_info))))
}
