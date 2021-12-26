pub mod user;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}
