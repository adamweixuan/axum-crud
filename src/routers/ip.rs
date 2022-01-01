use crate::cache::ip::CACHE;
use crate::entity::user::IPResponse;
use axum::http::StatusCode;
use axum::Json;

use chrono::prelude::*;

pub async fn get_ip() -> Result<Json<IPResponse>, StatusCode> {
    let ip = CACHE.get("origin");

    match ip {
        None => Err(StatusCode::NOT_FOUND),
        Some(ipadr) => Ok(Json(IPResponse {
            ip: ipadr.to_string(),
        })),
    }
}

pub async fn current_time() -> String {
    Local::now().to_string()
}
