use bigdecimal::BigDecimal;
use serde::Serialize;
#[derive(Serialize, Queryable)]
pub struct Product {
    pub id: i32,
    pub category_id: i32,
    pub name: String,
    pub description: String,
    pub price: BigDecimal,
    pub image_url: String,
    pub available: i32,
}

#[derive(Debug, DbEnum)]
#[DieselType = "User_role"]
pub enum UserRole {
    Client,
    Courier,
    Admin,
}

#[derive(Debug, DbEnum)]
#[DieselType = "Transport_colors"]
pub enum TransportColors {
    Black,
    Gray,
    White,
    Yellow,
    Red,
    Blue,
    Brow,
}

#[derive(Debug, DbEnum)]
#[DieselType = "Courier_status"]
pub enum CourierStatus {
    NotWorking,
    Free,
    Delivering,
    Returning,
}

#[derive(Debug, DbEnum)]
#[DieselType = "Order_status"]
pub enum OrderStatus {
    Shopping,
    Processing,
    Preparing,
    Delivering,
    Completed,
}
