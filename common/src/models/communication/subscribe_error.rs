use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName,
}
