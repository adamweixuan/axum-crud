use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use tokio::sync::OnceCell;
use tracing::debug;

static DB_POOL: OnceCell<MySqlPool> = OnceCell::const_new();

const DSN: &'static str = "mysql://your_db_name:your_db_passwd@your_db_host:port/axum_crud";

const POOL_SIZE: u32 = 1024;

async fn init_db() -> MySqlPool {
    let pool = MySqlPoolOptions::new()
        .max_connections(POOL_SIZE)
        .connect(DSN)
        .await
        .unwrap();

    debug!("init db success {:?}", pool);
    pool
}

pub async fn get_connection() -> &'static MySqlPool {
    DB_POOL.get_or_init(init_db).await
}
