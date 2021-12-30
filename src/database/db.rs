use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::env;
use tokio::sync::OnceCell;
use tracing::debug;

static DB_POOL: OnceCell<SqlitePool> = OnceCell::const_new();

const POOL_SIZE: u32 = 1024;

async fn init_sqlite() -> SqlitePool {
    let dsn = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:temp.db".to_string());
    let pool = SqlitePoolOptions::new()
        .max_connections(POOL_SIZE)
        .connect(&dsn)
        .await
        .unwrap();

    debug!("init db success {:?}", pool);
    pool
}

pub async fn get_connection() -> &'static SqlitePool {
    DB_POOL.get_or_init(init_sqlite).await
}
