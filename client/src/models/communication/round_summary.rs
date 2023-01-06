use serde::{Serialize, Deserialize};

use crate::models::challenge::ReportedChallengeResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}
