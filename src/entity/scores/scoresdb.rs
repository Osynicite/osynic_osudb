use crate::entity::scores::scores::Scores;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScoresDB {
    pub version: u32,
    pub beatmaps: Vec<Scores>,
}
