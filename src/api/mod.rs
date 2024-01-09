use std::sync::Arc;
use axum::Router;
use axum::routing::{delete, get, patch, post};
use crate::{controllers};
use crate::controllers::{auth_controller, user_controller};
use crate::repository::Repository;

#[derive(Clone)]
pub struct AppState {
    pub(crate) repository: Arc<Repository>,
}

pub async fn serve() -> Router {
    let state = AppState {
        repository: Arc::from(Repository::new().await)
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
        .route("/login", post(auth_controller::login))
        .route("/logout", get(auth_controller::logout));

    Router::new()
        .route("/ping", get(controllers::ping))
        .nest("/api", api)
        .with_state(state)
}