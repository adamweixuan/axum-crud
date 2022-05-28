use crate::routers::ip::{current_time, get_ip};
use crate::routers::user::{info, list_all, login, logout, register};
use axum::routing::post;
use axum::routing::{get, get_service};
use axum::{http, Router};
use tower_http::services::ServeDir;

pub mod cache;
pub mod database;
pub mod entity;
pub mod extract;
pub mod fileio;
pub mod infra;
pub mod pb;
pub mod routers;

pub async fn ping() -> &'static str {
    "pong"
}

pub fn router() -> Router {
    Router::new()
        .route("/ping", get(ping))
        .nest(
            "/user",
            Router::new()
                .route("/register", post(register))
                .route("/info", get(info))
                .route("/logout", post(logout))
                .route("/list", get(list_all))
                .route("/login", post(login)),
        )
        .nest(
            "/infra",
            Router::new()
                .route("/ip", get(get_ip))
                .route("/time", get(current_time)),
        )
        .nest(
            "static",
            get_service(ServeDir::new(".")).handle_error(|error: std::io::Error| async move {
                (
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("{}", error),
                )
            }),
        )
}
