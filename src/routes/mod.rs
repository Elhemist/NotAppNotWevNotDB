use crate::models::user;

pub mod cart;
pub mod orders;
pub mod products;
pub mod users;

#[derive(Debug)]
pub struct AuthorizedUser(user::User);
#[derive(Debug)]
pub struct AuthorizedAdmin(user::User);
