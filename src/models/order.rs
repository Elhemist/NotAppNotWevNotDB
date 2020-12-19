use serde::Serialize;

#[derive(Debug, DbEnum, Serialize)]
#[DieselType = "Order_status"]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Shopping,
    Processing,
    Preparing,
    Delivering,
    Completed,
}
