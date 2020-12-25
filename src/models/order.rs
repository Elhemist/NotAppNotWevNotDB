use crate::schema::addresses;
use crate::schema::orders;
use crate::schema::products_in_cart;
use crate::schema::products_in_orders;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

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
#[belongs_to(Address)]
#[table_name = "orders"]
pub struct Order {
    pub id: i32,
    pub user_id: Option<i32>,
    pub courier_id: Option<i32>,
    pub address_id: i32,
    pub status: OrderStatus,
    pub total_sum: BigDecimal,
    pub comment: Option<String>,
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

#[derive(Identifiable, Serialize, Insertable, Associations, Queryable, Debug)]
#[table_name = "products_in_orders"]
#[belongs_to(Order)]
#[belongs_to(super::product::Product)]
#[primary_key(order_id, product_id)]
pub struct ProductsInOrder {
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Serialize, Debug)]
pub struct OrderInfo {
    pub id: i32,
    pub status: OrderStatus,
    pub total_sum: BigDecimal,
    pub products: Vec<ProductInOrderInfo>,
    pub street: String,
    pub home: String,
    pub apartment: Option<String>,
    pub comment: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct ProductInOrderInfo {
    pub id: i32,
    pub quantity: i32,
}

#[derive(Identifiable, Associations, Queryable, Serialize, Deserialize, Debug)]
#[table_name = "addresses"]
#[belongs_to(super::user::User)]
pub struct Address {
    pub id: i32,
    pub user_id: i32,
    pub street: String,
    pub home: String,
    pub apartment: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "addresses"]
pub struct NewAddress {
    pub user_id: i32,
    pub street: String,
    pub home: String,
    pub apartment: Option<String>,
}
