use axum::{routing::get, routing::post, Router};
use axum_crud::ping;
use axum_crud::routers::user::{info, login, logout, register};
use mimalloc::MiMalloc;
use std::net::SocketAddr;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/ping", get(ping))
        .nest(
            "/user",
            Router::new()
                .route("/register", post(register))
                .route("/info", get(info))
                .route("/logout", post(logout))
                .route("/login", post(login)),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .http1_keepalive(true)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
