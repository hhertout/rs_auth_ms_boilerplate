use actix_web::{web, Scope};
use auth_controller::{check_cookie, check_token, login, logout};

pub mod auth_controller;
pub mod user_controller;

#[allow(dead_code)]
pub fn get_v1_service() -> Scope {
    web::scope("/api/v1")
        .service(login)
        .service(logout)
        .service(check_cookie)
        .service(check_token)
}
