use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashcashInput {
    pub complexity: u32,
    pub message: String,
}
