use serde::Serialize;

#[derive(Serialize, Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub phone: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: UserRole,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub session_id: Option<String>,
}

#[derive(Debug, DbEnum, Serialize)]
#[DieselType = "User_role"]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Client,
    Courier,
    Admin,
}
