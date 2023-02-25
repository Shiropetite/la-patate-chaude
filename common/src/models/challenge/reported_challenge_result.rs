use serde::{Deserialize, Serialize};

use super::ChallengeValue;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportedChallengeResult {
    name: String,
    value: ChallengeValue,
}
