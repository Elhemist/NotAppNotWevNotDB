use crate::models::order::ProductsInCart;
use crate::models::user::User;
use crate::schema::products_in_cart;
use crate::{errors::Error, schema};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn get_all(conn: &PgConnection, user: &User) -> Result<Vec<(i32, i32)>, Error> {
    ProductsInCart::belonging_to(user)
        .select((products_in_cart::product_id, products_in_cart::quantity))
        .load::<(i32, i32)>(conn)
        .map_err(|_| Error::DatabaseError)
}

pub fn add(conn: &PgConnection, user: &User, product_id: i32, quantity: i32) -> Result<(), Error> {
    diesel::insert_into(products_in_cart::table)
        .values(ProductsInCart {
            user_id: user.id,
            product_id,
            quantity,
        })
        .on_conflict((products_in_cart::user_id, products_in_cart::product_id))
        .do_update()
        .set(products_in_cart::quantity.eq(quantity))
        .execute(conn)
        .map_err(|_| Error::DatabaseError)
        .map(|_| ())
}

pub fn remove(conn: &PgConnection, user: &User, product_id: i32) -> Result<(), Error> {
    let filter = products_in_cart::table
        .filter(products_in_cart::user_id.eq(user.id))
        .filter(products_in_cart::product_id.eq(product_id));

    diesel::delete(filter)
        .execute(conn)
        .map_err(|_| Error::DatabaseError)
        .map(|_| ())
}
