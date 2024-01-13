use axum::Router;
use tokio::net::TcpListener;
use auth_api::config;
use crate::database::{Database, DatabaseService};

mod services;
mod api;
mod repository;
mod controllers;
mod database;


#[tokio::main]
async fn main() {
    env_logger::init();

    DatabaseService::new().migrations_migrate().await;

    config::account::create_super_admin_account().await;

    let port = std::env::var("PORT").unwrap_or_else(|_| String::from("4000"));
    let address = "0.0.0.0:".to_owned() + port.as_str();

    let app: Router = api::serve().await;
    let listener = TcpListener::bind(address)
        .await
        .unwrap();

    println!("📡 Server started ! Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
