use axum::{routing::get, routing::post, Router};
use axum_crud::cache::ip::background_run;
use axum_crud::ping;
use axum_crud::routers::ip::get_ip;
use axum_crud::routers::user::{info, list_all, login, logout, register};
use mimalloc::MiMalloc;
use std::net::SocketAddr;
use tracing::info;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();
    background_run();

    let app = Router::new()
        // `GET /` goes to `root`
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
        .nest("/infra", Router::new().route("/ip", get(get_ip)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .http1_keepalive(true)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
