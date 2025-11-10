use crate::entity::scores::scores::Scores;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScoresDB {
    pub version: u32,
    pub beatmaps: Vec<Scores>,
}
