use serde::{Serialize, Deserialize};

use super::ChallengeAnswer;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeResult {
    pub answer: ChallengeAnswer,
    pub next_target: String
}