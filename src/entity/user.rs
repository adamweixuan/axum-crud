use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, Default, FromRow)]
pub struct UserModel {
    pub id: i64,
    pub name: String,
    pub age: i64,
    pub email: String,
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct RegisterRequest {
    pub user_name: String,
    pub passwd: String,
    pub age: i64,
    pub email: String,
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct RegisterResponse {}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct ListResponse {
    pub user_list: Vec<UserModel>,
}
