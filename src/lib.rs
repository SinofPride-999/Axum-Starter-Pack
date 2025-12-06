pub mod routes;
pub mod controllers;
pub mod models;
pub mod middlewares;

use axum::Router;

pub fn create_app() -> Router {
    Router::new().merge(routes::app_routes())
}
