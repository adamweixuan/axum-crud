use axum_crud::cache::ip::background_run;
use axum_crud::router;
use clap::Parser;
use lazy_static::lazy_static;
use tracing::info;

// #[global_allocator]
// static GLOBAL: MiMalloc = MiMalloc;

lazy_static! {
    static ref VERSION: &'static str = {
        match option_env!("VERGEN_GIT_SEMVER_LIGHTWEIGHT") {
            Some(v) => v,
            None => env!("VERGEN_BUILD_SEMVER"),
        }
    };
    static ref LONG_VERSION: String = format!(
        "
Build Timestamp:     {}
Build Version:       {}
Commit SHA:          {:?}
Commit Date:         {:?}
Commit Branch:       {:?}
cargo Target Triple: {}
cargo Profile:       {}
cargo Features:      {}
",
        env!("VERGEN_BUILD_TIMESTAMP"),
        env!("VERGEN_BUILD_SEMVER"),
        option_env!("VERGEN_GIT_SHA"),
        option_env!("VERGEN_GIT_COMMIT_TIMESTAMP"),
        option_env!("VERGEN_GIT_BRANCH"),
        env!("VERGEN_CARGO_TARGET_TRIPLE"),
        env!("VERGEN_CARGO_PROFILE"),
        env!("VERGEN_CARGO_FEATURES")
    );
}

#[derive(Parser, Debug)]
#[clap(
    about,
    version(*VERSION),
    long_version(LONG_VERSION.as_str())
)]
struct AppOpt {
    #[clap(short, long, default_value_t = 8081)]
    port: u16,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_line_number(true)
        .pretty()
        .init();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    dotenv::dotenv().ok();
    let opt = AppOpt::parse();
    background_run();
    let addr = format!("0.0.0.0:{}", opt.port).parse().unwrap();
    info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .http1_keepalive(true)
        .serve(router().into_make_service())
        .await
        .unwrap();
}
