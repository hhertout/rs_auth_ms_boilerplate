use actix_web::{web, Scope};
use auth_controller::{check_cookie, check_token, login, logout};
use user_controller::{
    get_user_by_email, get_user_progression, hard_delete_user, remove_soft_deletion_user,
    save_user, soft_delete_user,
};

pub mod auth_controller;
pub mod user_controller;

#[allow(dead_code)]
pub fn get_v1_service() -> Scope {
    web::scope("/api/v1")
        .service(login)
        .service(logout)
        .service(check_cookie)
        .service(check_token)
        .service(save_user)
        .service(get_user_by_email)
        .service(get_user_progression)
        .service(soft_delete_user)
        .service(remove_soft_deletion_user)
        .service(hard_delete_user)
}
