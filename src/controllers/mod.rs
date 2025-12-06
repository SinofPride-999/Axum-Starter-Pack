use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    pub message: String,
}

pub async fn home() -> Json<Message> {
    Json(Message {
        message: "Welcome to Axum Starter Pack!".to_string(),
    })
}

pub async fn hello() -> &'static str {
    "Hello from Axum!"
}
