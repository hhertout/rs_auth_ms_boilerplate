use std::sync::Arc;
use axum::Router;
use axum::routing::{get, patch, post};
use crate::{controllers};
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
        .route("/user/new", post(controllers::user_controller::save_user))
        .route("/user/find-one", get(controllers::user_controller::get_user_by_email))
        .route("/user/password/update", patch(controllers::user_controller::update_password))
        .route("/login", post(controllers::auth_controller::login));

    Router::new()
        .route("/ping", get(controllers::ping))
        .nest("/api", api)
        .with_state(state)
}