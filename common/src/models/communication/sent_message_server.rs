use serde::{Serialize, Deserialize};

use super::{Welcome};

#[derive(Debug, Serialize, Deserialize)]
pub enum SentMessageServer {
    Welcome(Welcome),
}