use serde::{Deserialize, Serialize};

use super::Subscribe;

#[derive(Debug, Serialize, Deserialize)]
pub enum ReceivedMessageServer {
    Hello,
    Subscribe(Subscribe),
}
