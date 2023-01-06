use serde::{Serialize, Deserialize};

use crate::PublicPlayer;

#[derive(Debug, Serialize, Deserialize)]
pub struct EndOfGame {
    leader_board: Vec<PublicPlayer>
}