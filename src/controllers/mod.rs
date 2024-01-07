use axum::Json;
use serde::{Deserialize, Serialize};

pub mod user_controller;

#[derive(Serialize, Deserialize)]
pub struct PingResponse {
    message: String,
}

pub async fn ping() -> Json<PingResponse> {
    Json(PingResponse {
        message: String::from("Pong")
    })
}