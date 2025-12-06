use axum::{routing::get, Router};
use crate::controllers;

pub fn app_routes() -> Router {
    Router::new()
        .route("/", get(controllers::home))
        .route("/hello", get(controllers::hello))
}
