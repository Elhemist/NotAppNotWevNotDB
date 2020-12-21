use rocket_contrib::databases;

pub mod products;
pub mod users;
#[databases::database("pg")]
pub struct Conn(databases::diesel::PgConnection);
