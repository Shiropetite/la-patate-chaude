use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashcashOutput {
    pub seed: u64,
    pub hashcode: String
}