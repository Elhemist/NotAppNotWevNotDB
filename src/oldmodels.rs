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
