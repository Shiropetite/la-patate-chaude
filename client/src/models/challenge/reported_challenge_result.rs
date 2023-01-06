use serde::{Serialize, Deserialize};

use crate::ChallengeValue;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportedChallengeResult {
    name: String,
    value: ChallengeValue
}