use axum::{http::StatusCode, Json};
use serde_json::json;

pub fn uh_oh() -> (StatusCode, axum::Json<serde_json::Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "Error": "Welp, This isn't supposed to happen, report it to our developer team please and thank you :D",
        })),
    )
}
