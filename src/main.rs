use std::sync::Arc;

use crate::database::{Database, DatabaseService};
use actix_web::{web, App, HttpServer};
use auth_api::{config, controllers::ping};
use controllers::{v1::get_v1_service, AppState};
use log::info;
use repository::Repository;
use services::access_control::AccessControl;

mod controllers;
mod database;
mod repository;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    DatabaseService::new().migrations_migrate().await;

    config::account::create_super_admin_account().await;

    let state = AppState {
        repository: Arc::from(Repository::new().await),
        access_control: Arc::from(AccessControl::new().await),
    };

    let port = std::env::var("PORT").unwrap_or_else(|_| String::from("4000"));
    let ipv4 = "0.0.0.0";

    info!("📡 Server started ! Listening on {}:{}", ipv4, port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(ping)
            .service(get_v1_service())
    })
    .bind((ipv4, port.parse::<u16>().unwrap()))?
    .run()
    .await
}
