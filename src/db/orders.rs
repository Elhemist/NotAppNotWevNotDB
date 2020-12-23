use crate::errors::Error;
use crate::models::order::Order;
use crate::models::product::Product;
use crate::models::user::User;
use crate::schema::products;
use bigdecimal::BigDecimal;
use diesel::prelude::*;

use diesel::pg::PgConnection;
pub fn create(conn: &PgConnection, user: &User) -> Result<Order, Error> {
    let cart = super::cart::get(conn, user).unwrap();

    let products = products::table
        .filter(products::id.eq_any(cart.iter().map(|item| item.0)))
        .load::<Product>(conn)
        .map_err(|_| Error::DatabaseError)?;

    let mut total_sum = BigDecimal::from(0);

    for (product_id, quantity) in cart {
        let product = match products.iter().find(|product| product.id == product_id) {
            Some(product) => product,
            None => continue,
        };

        total_sum += &product.price * BigDecimal::from(quantity)
    }

    println!("{}", total_sum);

    Err(Error::Unknown)
}
