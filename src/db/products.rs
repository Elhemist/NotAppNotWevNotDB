use crate::models::product::Product;
use crate::schema::products;
use crate::{errors::Error, schema};
use bigdecimal::BigDecimal;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Insertable, Deserialize)]
#[table_name = "products"]
pub struct NewProduct {
    pub category_id: i32,
    pub name: String,
    pub description: String,
    pub price: BigDecimal,
    pub image_url: String,
    pub available: i32,
}

pub fn get_all(conn: &PgConnection) -> Result<Vec<Product>, Error> {
    schema::products::table
        .load::<Product>(conn)
        .map_err(Error::from)
}

pub fn get(conn: &PgConnection, id: i32) -> Result<Product, Error> {
    schema::products::table
        .find(id)
        .first::<Product>(conn)
        .map_err(Error::from)
}

pub fn create(conn: &PgConnection, new_product: NewProduct) -> Result<Product, Error> {
    diesel::insert_into(products::table)
        .values(new_product)
        .get_result::<Product>(conn)
        .map_err(Error::from)
}
