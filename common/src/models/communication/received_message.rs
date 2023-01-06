use serde::{Serialize, Deserialize};

use crate::models::challenge::{Challenge, ChallengeTimeout};

use super::{Welcome, SubscribeResult, PublicPlayer, RoundSummary, EndOfGame};

#[derive(Debug, Serialize, Deserialize)]
pub enum ReceivedMessage {
    Welcome(Welcome),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge),
    ChallengeTimeout(ChallengeTimeout),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame)
}