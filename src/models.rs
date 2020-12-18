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

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub phone: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: UserRole,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
}

use super::schema::users;
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub phone: String,
    pub password_hash: String,
    pub role: UserRole,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, DbEnum, Serialize)]
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
