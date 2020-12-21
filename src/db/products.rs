use crate::models::product::Product;
use crate::{errors::Error, schema};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

pub fn get_all(conn: &PgConnection) -> Result<Vec<Product>, Error> {
    schema::products::table
        .load::<Product>(conn)
        .map_err(|_| Error::DatabaseError)
}

pub fn get(conn: &PgConnection, id: i32) -> Result<Product, Error> {
    schema::products::table
        .find(id)
        .first::<Product>(conn)
        .map_err(|e| match e {
            DieselError::NotFound => Error::NotFound,
            _ => Error::DatabaseError,
        })
}
