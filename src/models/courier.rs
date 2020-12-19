#[derive(Debug, DbEnum)]
#[DieselType = "Courier_status"]
pub enum CourierStatus {
    NotWorking,
    Free,
    Delivering,
    Returning,
}
