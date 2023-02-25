use serde::{Deserialize, Serialize};

use super::Welcome;

#[derive(Debug, Serialize, Deserialize)]
pub enum SentMessageServer {
    Welcome(Welcome),
}
