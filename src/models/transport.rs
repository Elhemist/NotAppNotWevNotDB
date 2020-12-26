use serde::Serialize;

#[derive(Debug, DbEnum, Serialize)]
#[DieselType = "Transport_colors"]
#[serde(rename_all = "lowercase")]
pub enum TransportColors {
    Black,
    Gray,
    White,
    Yellow,
    Red,
    Blue,
    Brow,
}
