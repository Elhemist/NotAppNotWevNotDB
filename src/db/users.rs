use crate::models::user::{User, UserRole};
use crate::schema::users;
use crate::{errors::Error, schema};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error as DieselError;
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
    match diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(conn)
    {
        Ok(user) => Ok(user),
        Err(error) => {
            if let DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = error {
                if info.constraint_name() == Some("unique_phone") {
                    return Err(Error::PhoneAlreadyInUse);
                }
            }
            Err(Error::Unknown)
        }
    }
}

pub fn login(conn: &PgConnection, phone: String, password: String) -> Result<User, Error> {
    let user = match schema::users::table
        .filter(schema::users::phone.eq(phone))
        .limit(1)
        .load::<User>(conn)
    {
        Ok(mut user) => {
            if let Some(user) = user.pop() {
                user
            } else {
                return Err(Error::UserNotFound);
            }
        }
        Err(error) => {
            if let DieselError::NotFound = error {
                return Err(Error::UserNotFound);
            }
            return Err(Error::Unknown);
        }
    };

    let mut hasher = Sha3_256::new();
    hasher.update(password);
    let password_hash = hex::encode(hasher.finalize());

    if user.password_hash != password_hash {
        return Err(Error::InvalidPassword);
    }

    let session_id = create_session_id();
    let user = diesel::update(schema::users::table.find(user.id))
        .set(schema::users::session_id.eq(session_id))
        .get_result::<User>(conn)
        .map_err(|_| Error::Unknown)?;

    Ok(user)
}

fn create_session_id() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(30).collect()
}

pub fn logout(conn: &PgConnection, user_id: i32) -> Result<(), Error> {
    diesel::update(schema::users::table.find(user_id))
        .set(schema::users::session_id.eq::<Option<String>>(None))
        .execute(conn)
        .map_err(|_| Error::Unknown)
        .map(|_| ())
}

pub fn user_by_session_id(conn: &PgConnection, session_id: String) -> Result<User, Error> {
    let user = match schema::users::table
        .filter(schema::users::session_id.eq(session_id))
        .limit(1)
        .load::<User>(conn)
    {
        Ok(mut user) => {
            if let Some(user) = user.pop() {
                user
            } else {
                return Err(Error::InvalidSessionId);
            }
        }
        Err(error) => {
            if let DieselError::NotFound = error {
                return Err(Error::InvalidSessionId);
            }
            return Err(Error::Unknown);
        }
    };

    Ok(user)
}
