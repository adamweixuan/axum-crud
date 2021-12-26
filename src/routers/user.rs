use crate::database::user::{create, list_user};
use crate::entity::user::{ListResponse, RegisterRequest, RegisterResponse, UserModel};
use crate::entity::Pagination;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::Json;
use tracing::debug;

pub async fn register(
    Json(req): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, StatusCode> {
    let user = UserModel {
        id: 0,
        name: req.user_name,
        age: req.age,
        email: req.email,
        address: req.address,
    };

    debug!("register req {:?}", &user);

    create(&user)
        .await
        .map(|_| Json(RegisterResponse {}))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn list_all(
    pagination: Option<Query<Pagination>>,
) -> Result<Json<ListResponse>, StatusCode> {
    let Query(pagination) = pagination.unwrap_or_default();

    let size = pagination.limit.unwrap_or(10);
    let offset = pagination.offset.unwrap_or(0);

    debug!("register req {}->{}", size, offset);

    let rsp = list_user(size as u32, offset as u32)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .ok();

    if rsp.is_none() {
        Ok(Json(ListResponse {
            user_list: Vec::new(),
        }))
    } else {
        Ok(Json(ListResponse {
            user_list: rsp.unwrap(),
        }))
    }
}

pub async fn login() {}

pub async fn logout() {}

pub async fn info() -> &'static str {
    "info..."
}
