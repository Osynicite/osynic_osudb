use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq,Serialize, Deserialize)]
pub struct Collection {
    pub name: Option<String>,
    pub beatmap_hashes: Vec<Option<String>>,
}