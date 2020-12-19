use rocket_contrib::databases;

pub mod users;
#[databases::database("pg")]
pub struct Conn(databases::diesel::PgConnection);
