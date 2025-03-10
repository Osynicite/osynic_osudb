//! Parsing for the `collection.db` file, containing all user collections.
use crate::entity::collection::collection::Collection;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CollectionDB {
    pub version: u32,
    pub collections: Vec<Collection>,
}
