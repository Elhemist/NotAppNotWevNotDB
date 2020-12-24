use crate::errors::Error;
use crate::models::user::{User, UserRole};
use crate::schema::users;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sha3::{Digest, Sha3_256};

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub phone: String,
    pub password_hash: String,
    pub role: UserRole,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
}

pub fn create(conn: &PgConnection, new_user: NewUser) -> Result<User, Error> {
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(conn)
        .map_err(Error::from)
}

pub fn login(conn: &PgConnection, phone: String, password: String) -> Result<User, Error> {
    let user = users::table
        .filter(users::phone.eq(phone))
        .first::<User>(conn)?;

    let mut hasher = Sha3_256::new();
    hasher.update(password);
    let password_hash = hex::encode(hasher.finalize());

    if user.password_hash != password_hash {
        return Err(Error::InvalidPassword);
    }

    let session_id = create_session_id();
    let user = diesel::update(users::table.find(user.id))
        .set(users::session_id.eq(session_id))
        .get_result::<User>(conn)
        .map_err(Error::from)?;

    Ok(user)
}

fn create_session_id() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(30).collect()
}

pub fn logout(conn: &PgConnection, user_id: i32) -> Result<(), Error> {
    diesel::update(users::table.find(user_id))
        .set(users::session_id.eq::<Option<String>>(None))
        .execute(conn)
        .map_err(Error::from)
        .map(|_| ())
}

pub fn user_by_session_id(conn: &PgConnection, session_id: String) -> Result<User, Error> {
    users::table
        .filter(users::session_id.eq(session_id))
        .first::<User>(conn)
        .map_err(Error::from)
}

pub fn user_by_id(conn: &PgConnection, id: i32) -> Result<User, Error> {
    users::table
        .find(id)
        .first::<User>(conn)
        .map_err(Error::from)
}
