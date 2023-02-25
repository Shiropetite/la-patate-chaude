use serde::{Deserialize, Serialize};

use super::md5::MD5HashcashOutput;

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashcashOutput),
}
