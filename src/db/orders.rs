use crate::errors::Error;
use crate::models::order::{Order, OrderInfo, OrderStatus, ProductInOrderInfo, ProductsInOrder};
use crate::models::product::Product;
use crate::models::user::User;
use crate::schema::orders;
use crate::schema::products;
use crate::schema::products_in_orders;
use bigdecimal::BigDecimal;
use diesel::prelude::*;

use diesel::pg::PgConnection;

#[derive(Insertable)]
#[table_name = "orders"]
pub struct NewOrder {
    pub user_id: Option<i32>,
    pub courier_id: Option<i32>,
    pub address_id: Option<i32>,
    pub status: OrderStatus,
    pub total_sum: BigDecimal,
    pub comment: Option<String>,
}

pub fn create(conn: &PgConnection, user: &User) -> Result<OrderInfo, Error> {
    conn.transaction::<_, Error, _>(|| {
        let cart = super::cart::get(conn, user).unwrap();

        if cart.is_empty() {
            return Err(Error::CartIsEmpty);
        }

        let products = products::table
            .filter(products::id.eq_any(cart.iter().map(|item| item.0)))
            .load::<Product>(conn)?;

        let mut total_sum = BigDecimal::from(0);

        for (product_id, quantity) in &cart {
            let product = match products.iter().find(|product| product.id == *product_id) {
                Some(product) => product,
                None => continue,
            };

            total_sum += &product.price * BigDecimal::from(*quantity)
        }

        let new_order = NewOrder {
            user_id: Some(user.id),
            courier_id: None,
            address_id: None,
            status: OrderStatus::Processing,
            total_sum,
            comment: None,
        };

        let new_order = diesel::insert_into(orders::table)
            .values(new_order)
            .get_result::<Order>(conn)?;

        let products_in_order = cart
            .iter()
            .map(|item| ProductsInOrder {
                order_id: new_order.id,
                product_id: item.0,
                quantity: item.1,
            })
            .collect::<Vec<_>>();

        diesel::insert_into(products_in_orders::table)
            .values(products_in_order)
            .execute(conn)?;

        super::cart::clear(conn, user)?;

        let products_in_order_info = cart
            .iter()
            .map(|product| ProductInOrderInfo {
                id: product.0,
                quantity: product.1,
            })
            .collect::<Vec<_>>();

        let info = OrderInfo {
            id: new_order.id,
            status: new_order.status,
            total_sum: new_order.total_sum,
            products: products_in_order_info,
        };

        Ok(info)
    })
}

pub fn get(conn: &PgConnection, id: i32) -> Result<OrderInfo, Error> {
    let order = orders::table.find(id).first::<Order>(conn)?;

    let products_in_order_info = ProductsInOrder::belonging_to(&order)
        .load::<ProductsInOrder>(conn)?
        .iter()
        .map(|product| ProductInOrderInfo {
            id: product.product_id,
            quantity: product.quantity,
        })
        .collect::<Vec<_>>();

    let info = OrderInfo {
        id: order.id,
        status: order.status,
        total_sum: order.total_sum,
        products: products_in_order_info,
    };

    Ok(info)
}

pub fn history(conn: &PgConnection, user: &User) -> Result<Vec<OrderInfo>, Error> {
    let user_orders = Order::belonging_to(user).load::<Order>(conn)?;

    let products = ProductsInOrder::belonging_to(&user_orders)
        .load::<ProductsInOrder>(conn)?
        .grouped_by(&user_orders)
        .iter()
        .map(|products| {
            products
                .iter()
                .map(|product| ProductInOrderInfo {
                    id: product.product_id,
                    quantity: product.quantity,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    let orders_info = user_orders
        .into_iter()
        .zip(products)
        .collect::<Vec<_>>()
        .into_iter()
        .map(|(order, products)| OrderInfo {
            id: order.id,
            status: order.status,
            total_sum: order.total_sum,
            products,
        })
        .collect::<Vec<_>>();

    Ok(orders_info)
}
