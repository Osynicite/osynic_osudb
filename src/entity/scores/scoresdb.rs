use serde::{Deserialize, Serialize};
use crate::entity::scores::scores::Scores;

#[derive(Clone, Debug, PartialEq,Serialize, Deserialize)]
pub struct ScoresDB {
    pub version: u32,
    pub beatmaps: Vec<Scores>,
}