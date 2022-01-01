use axum_crud::cache::ip::background_run;
use axum_crud::router;
use mimalloc::MiMalloc;
use tracing::info;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    dotenv::dotenv().ok();

    background_run();
    let addr = std::env::var("HOST_PORT")
        .unwrap_or_else(|_| "0.0.0.0:8001".to_string())
        .parse()
        .unwrap();

    info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .http1_keepalive(true)
        .serve(router().into_make_service())
        .await
        .unwrap();
}
