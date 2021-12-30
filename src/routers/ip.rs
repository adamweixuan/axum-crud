use crate::cache::ip::CACHE;
use crate::entity::user::IPResponse;
use axum::http::StatusCode;
use axum::Json;

pub async fn get_ip() -> Result<Json<IPResponse>, StatusCode> {
    let ip = CACHE.get("origin");

    match ip {
        None => Err(StatusCode::NOT_FOUND),
        Some(ipadr) => Ok(Json(IPResponse {
            ip: ipadr.to_string(),
        })),
    }
}
