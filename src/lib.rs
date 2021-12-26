pub mod database;
pub mod entity;
pub mod routers;

pub async fn ping() -> &'static str {
    "pong"
}
