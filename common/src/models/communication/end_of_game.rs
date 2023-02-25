use serde::{Deserialize, Serialize};

use super::PublicPlayer;

#[derive(Debug, Serialize, Deserialize)]
pub struct EndOfGame {
    leader_board: Vec<PublicPlayer>,
}
