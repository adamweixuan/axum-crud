use std::time::Duration;

use lazy_static::lazy_static;
use tracing::trace;

const TEN_SECOND: u64 = 10000;

const DEFAULT_HTTP_TIMEOUT: Duration = Duration::from_millis(TEN_SECOND);

lazy_static! {
    pub static ref HTTP_CLIENT: reqwest::Client = init_http_cli();
}

fn init_http_cli() -> reqwest::Client {
    trace!("init_http_cli...");
    reqwest::ClientBuilder::default()
        .timeout({
            if let Ok(env_str) = std::env::var("HTTP_TOTAL_TIMEOUT") {
                Duration::from_millis(env_str.parse::<u64>().unwrap_or(TEN_SECOND))
            } else {
                DEFAULT_HTTP_TIMEOUT
            }
        })
        .build()
        .expect("should be able to build a http client")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::time::Instant;

    #[tokio::test]
    async fn test_init_http_cli() {
        dotenv::dotenv().ok();
        tracing_subscriber::fmt::init();
        println!("init_http_cli {:?}", *HTTP_CLIENT);

        let start = Instant::now();

        let rsp = HTTP_CLIENT
            .get("https://httpbin.org/ip")
            .send()
            .await
            .unwrap()
            .json::<HashMap<String, String>>()
            .await
            .unwrap();

        println!("response {:?}\ttime cost {:?}", rsp, start.elapsed());
    }
}
