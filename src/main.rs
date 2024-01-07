use axum::Router;
use tokio::net::TcpListener;
use crate::database::Database;

mod services;
mod api;
mod repository;
mod controllers;
mod database;


#[tokio::main]
async fn main() {
    env_logger::init();

    Database::new().migrations_migrate().await;

    let app: Router = api::serve().await;
    let listener = TcpListener::bind("0.0.0.0:4000")
        .await
        .unwrap();

    println!("📡 Server started ! Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
