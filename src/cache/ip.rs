use crate::infra::httpcli::HTTP_CLIENT;
use dashmap::DashMap;
use lazy_static::lazy_static;
use std::collections::HashMap;

use crossbeam_channel::tick;
use std::time::Duration;
use tracing::{debug, error};

const DEFAULT_JOB_INTERVAL: Duration = Duration::from_secs(10);

lazy_static! {
    pub static ref CACHE: DashMap<String, String> = DashMap::new();
}

async fn fetch_ip() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let rsp = HTTP_CLIENT
        .get("https://httpbin.org/ip")
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    debug!("fetch_ip resp {:?}", &rsp);
    Ok(rsp)
}

async fn update_cache() {
    let result = fetch_ip().await;
    match result {
        Ok(ipdata) => {
            CACHE.clear();
            for (k, v) in ipdata {
                CACHE.insert(k, v);
            }
            debug!("background_run success {:?}", CACHE.len());
        }
        Err(err) => {
            error!("fetch error {:?}", err);
        }
    }
}

pub fn background_run() {
    let ticker = tick({
        if let Ok(env_str) = std::env::var("JOB_INTERVAL_SECOND") {
            debug!("JOB_INTERVAL_SECOND {}", env_str);
            Duration::from_secs(env_str.parse::<u64>().unwrap_or(10))
        } else {
            DEFAULT_JOB_INTERVAL
        }
    });

    tokio::spawn(async move {
        loop {
            let _msg = ticker.recv().unwrap();
            update_cache().await
        }
    });
}
