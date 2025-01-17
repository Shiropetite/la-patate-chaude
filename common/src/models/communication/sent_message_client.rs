use serde::{Deserialize, Serialize};

use crate::models::challenge::ChallengeResult;

use super::Subscribe;

#[derive(Debug, Serialize, Deserialize)]
pub enum SentMessageClient {
    Hello,
    Subscribe(Subscribe),
    ChallengeResult(ChallengeResult),
}
