pub mod cache;
pub mod database;
pub mod entity;
pub mod infra;
pub mod routers;

pub async fn ping() -> &'static str {
    "pong"
}
