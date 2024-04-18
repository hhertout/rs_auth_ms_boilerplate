use std::sync::Arc;
use axum::Router;
use axum::routing::{delete, get, patch, post};
use crate::controllers;
use crate::controllers::{auth_controller, user_controller};
use crate::repository::Repository;
use crate::services::access_control::AccessControl;
use crate::config;

#[derive(Clone)]
pub struct AppState {
    pub(crate) repository: Arc<Repository>,
    pub(crate) access_control: Arc<AccessControl>
}

pub async fn serve() -> Router {
    let state = AppState {
        repository: Arc::from(Repository::new().await),
        access_control: Arc::from(AccessControl::new().await),
    };

    let api = Router::new()
        .route("/auth/check-token", get(auth_controller::check_token))
        .route("/auth/check-cookie", get(auth_controller::check_cookie))
        .route("/user/new", post(user_controller::save_user))
        .route("/user/find-one", get(user_controller::get_user_by_email))
        .route("/user/password/update", patch(user_controller::update_password))
        .route("/user/ban", delete(user_controller::soft_delete_user))
        .route("/user/unban", patch(user_controller::remove_soft_deletion_user))
        .route("/user/delete", delete(user_controller::hard_delete_user))
        .route("/user/progression", get(user_controller::get_user_progression))
        .route("/login", post(auth_controller::login))
        .route("/logout", get(auth_controller::logout))
        .route("/auth/csrf-token", get(auth_controller::get_csrf_token));

    Router::new()
        .route("/ping", get(controllers::ping))
        .nest("/api/v1", api)
        .layer(config::cors::cors_layer())
        .with_state(state)
}
