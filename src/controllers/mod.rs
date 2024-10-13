use std::sync::Arc;

use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{repository::Repository, services::access_control::AccessControl};

pub(crate) mod v1;

#[derive(Clone)]
pub struct AppState {
    pub(crate) repository: Arc<Repository>,
    pub(crate) access_control: Arc<AccessControl>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CustomResponse {
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct PingResponse {
    message: String,
}

#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().json(CustomResponse {
        message: String::from("Pong"),
    })
}
