use axum::{http::StatusCode, Json};
use serde::Serialize;
use serde_json::json;

pub fn error_response(status: StatusCode, message: &str) -> (StatusCode, Json<serde_json::Value>) {
    let err_msg = json!({
        "status": "error",
        "message": message,
    });
    (status, Json(err_msg))
}

pub fn success_response(
    status: StatusCode,
    message: &str,
) -> (StatusCode, Json<serde_json::Value>) {
    let success_msg = json!({
        "status": "success",
        "message": message,
    });

    (status, Json(success_msg))
}

pub fn success_response_with_data<T: Serialize>(
    status: StatusCode,
    message: &str,
    data: Option<T>,
) -> (StatusCode, Json<serde_json::Value>) {
    let mut success_msg = json!({
        "status": "success",
        "message": message,
    });
    if let Some(data) = data {
        success_msg["data"] = json!(data);
    }
    (status, Json(success_msg))
}
