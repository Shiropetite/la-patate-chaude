use serde::{Serialize, Deserialize};

use super::ChallengeValue;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportedChallengeResult {
    name: String,
    value: ChallengeValue
}