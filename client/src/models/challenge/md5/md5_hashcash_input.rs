use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashcashInput {
    pub complexity: u32,
    pub message: String
}