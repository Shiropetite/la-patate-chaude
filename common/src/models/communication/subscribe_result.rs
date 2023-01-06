use serde::{Serialize, Deserialize};

use super::SubscribeError;

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeResult {
    Ok, Err(SubscribeError)
}
