use serde::{Serialize, Deserialize};

use crate::ChallengeAnswer;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeResult {
    pub answer: ChallengeAnswer,
    pub next_target: String
}