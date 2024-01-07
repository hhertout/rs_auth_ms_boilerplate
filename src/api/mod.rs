use std::sync::Arc;
use axum::Router;
use axum::routing::{get, post};
use crate::{controllers};
use crate::repository::Repository;

#[derive(Clone)]
pub struct AppState {
    pub(crate) repository: Arc<Repository>,
}

pub async fn serve() -> Router {
    Router::new()
        .route("/ping", get(controllers::ping))
        .route("/api/user/save", post(controllers::user_controller::save_user))

        .with_state(AppState {
            repository: Arc::from(Repository::new().await)
        })
}