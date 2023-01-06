use serde::{Serialize, Deserialize};

use super::md5::MD5HashcashOutput;

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashcashOutput),
}