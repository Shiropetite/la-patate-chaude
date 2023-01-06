use serde::{Serialize, Deserialize};

use crate::models::challenge::ChallengeResult;

use super::Subscribe;

#[derive(Debug, Serialize, Deserialize)]
pub enum SentMessage {
    Hello,
    Subscribe(Subscribe),
    ChallengeResult(ChallengeResult),
}