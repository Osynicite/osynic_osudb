use crate::entity::scores::field::replay::Replay;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Scores {
    pub hash: Option<String>,
    pub scores: Vec<Replay>,
}
