use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, sqlx::FromRow)]
pub struct UserModel {
    pub id: uuid::Uuid,
    pub name: String,
    pub age: i32,
    pub email: String,
    pub address: String,
}
