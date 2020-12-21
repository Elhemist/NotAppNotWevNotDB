use crate::schema::orders;
use crate::schema::products_in_cart;
use bigdecimal::BigDecimal;
use serde::Serialize;

#[derive(Debug, DbEnum, Serialize)]
#[DieselType = "Order_status"]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Shopping,
    Processing,
    Preparing,
    Delivering,
    Completed,
}

#[derive(Identifiable, Serialize, Associations, Queryable, Debug)]
#[belongs_to(super::user::User)]
#[table_name = "orders"]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub courier_id: i32,
    pub address_id: i32,
    pub status: OrderStatus,
    pub total_sum: BigDecimal,
    pub comment: String,
}

#[derive(Identifiable, Serialize, Insertable, Associations, Queryable, Debug)]
#[table_name = "products_in_cart"]
#[belongs_to(super::user::User)]
#[belongs_to(super::product::Product)]
#[primary_key(user_id, product_id)]
pub struct ProductsInCart {
    pub user_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}
