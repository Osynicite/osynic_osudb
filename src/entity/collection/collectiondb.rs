//! Parsing for the `collection.db` file, containing all user collections.
use crate::entity::collection::collection::Collection;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CollectionDB {
    pub version: u32,
    pub collections: Vec<Collection>,
}
