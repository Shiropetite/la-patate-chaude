use serde::{Deserialize, Serialize};

use crate::models::challenge::{Challenge, ChallengeResult, ChallengeTimeout};

use super::{EndOfGame, PublicPlayer, RoundSummary, SubscribeResult, Welcome};

#[derive(Debug, Serialize, Deserialize)]
pub enum ReceivedMessageClient {
    Welcome(Welcome),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge),
    ChallengeResult(ChallengeResult),
    ChallengeTimeout(ChallengeTimeout),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame),
}
