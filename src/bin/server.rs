use axum_crud::cache::ip::background_run;
use axum_crud::router;
use clap::Parser;
use mimalloc::MiMalloc;
use tracing::info;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct AppOpt {
    #[clap(short, long, default_value_t = 8081)]
    port: u16,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    dotenv::dotenv().ok();
    let opt = AppOpt::parse();
    background_run();
    // let addr = std::env::var("HOST_PORT")
    //     .unwrap_or_else(|_| "0.0.0.0:8001".to_string())
    //     .parse()
    //     .unwrap();
    let addr = format!("0.0.0.0:{}", opt.port).parse().unwrap();
    info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .http1_keepalive(true)
        .serve(router().into_make_service())
        .await
        .unwrap();
}
