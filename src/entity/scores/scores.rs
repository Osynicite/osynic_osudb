use crate::entity::scores::field::replay::Replay;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Scores {
    pub hash: Option<String>,
    pub scores: Vec<Replay>,
}
