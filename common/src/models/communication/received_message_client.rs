use serde::{Serialize, Deserialize};

use crate::models::challenge::{Challenge, ChallengeTimeout, ChallengeResult};

use super::{Welcome, SubscribeResult, PublicPlayer, RoundSummary, EndOfGame};

#[derive(Debug, Serialize, Deserialize)]
pub enum ReceivedMessageClient {
    Welcome(Welcome),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge),
    ChallengeResult(ChallengeResult),
    ChallengeTimeout(ChallengeTimeout),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame)
}