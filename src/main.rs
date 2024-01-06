use axum::{Json, Router};
use axum::routing::get;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Message {
    message: String,
}

async fn handler() -> Json<Message> {
    let message = Message { message: String::from("toto") };
    return Json(message);
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();
    println!("📡 Server started ! Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
