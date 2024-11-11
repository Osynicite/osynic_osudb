use serde::{Deserialize, Serialize};
use crate::entity::scores::field::replay::Replay;

#[derive(Clone, Debug, PartialEq,Serialize, Deserialize)]
pub struct Scores {
    pub hash: Option<String>,
    pub scores: Vec<Replay>,
}