use crate::routers::ip::{current_time, get_ip};
use crate::routers::user::{info, list_all, login, logout, register};
use axum::routing::get;
use axum::routing::post;
use axum::Router;

pub mod cache;
pub mod database;
pub mod entity;
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
}
