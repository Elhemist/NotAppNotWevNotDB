use rocket_contrib::json::JsonValue;
use serde::Serialize;
#[derive(Serialize)]
pub struct ResponseData<T> {
    pub error: bool,
    // #[serde(default = "SUCCESS")]
    pub code: JsonValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ResponseData<T> {
    pub fn success(data: Option<T>) -> ResponseData<T> {
        ResponseData {
            error: false,
            code: json!(String::from("SUCCESS")),
            data,
        }
    }
}
