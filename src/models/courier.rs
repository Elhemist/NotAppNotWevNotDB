use crate::schema::courier;
use serde::Serialize;

#[derive(Debug, DbEnum, Serialize)]
#[DieselType = "Courier_status"]
#[serde(rename_all = "lowercase")]
pub enum CourierStatus {
    NotWorking,
    Free,
    Delivering,
    Returning,
}

#[derive(Identifiable, Associations, Serialize, Queryable, Debug)]
#[table_name = "courier"]
#[belongs_to(super::user::User)]
#[primary_key(user_id)]
pub struct Courier {
    pub user_id: i32,
    pub transport_id: Option<i32>,
    pub status: CourierStatus,
}
