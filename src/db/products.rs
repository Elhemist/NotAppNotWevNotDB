use crate::models::product::Product;
use crate::schema::products;
use crate::{errors::Error, schema};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error as DieselError;

pub fn get_all(conn: &PgConnection) -> Result<Vec<Product>, Error> {
    schema::products::table
        .load::<Product>(conn)
        .map_err(|_| Error::DatabaseError)
}
