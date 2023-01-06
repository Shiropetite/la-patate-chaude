use serde::{Serialize, Deserialize};

use super::PublicPlayer;

#[derive(Debug, Serialize, Deserialize)]
pub struct EndOfGame {
    leader_board: Vec<PublicPlayer>
}