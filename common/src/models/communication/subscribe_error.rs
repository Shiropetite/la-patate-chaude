use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum  SubscribeError { AlreadyRegistered, InvalidName }