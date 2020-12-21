use crate::schema::{product_category, products};
use serde::Serialize;

#[derive(Identifiable, Serialize, Associations, Queryable, Debug)]
#[belongs_to(Category)]
#[table_name = "products"]
pub struct Product {
    pub id: i32,
    pub category_id: i32,
    pub name: String,
    pub description: String,
    pub price: bigdecimal::BigDecimal,
    pub image_url: String,
    pub available: i32,
}

#[derive(Identifiable, Serialize, Queryable, Debug)]
#[table_name = "product_category"]
pub struct Category {
    pub id: i32,
    pub name: String,
}
