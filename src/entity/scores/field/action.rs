use crate::entity::scores::field::button::{ManiaButtonSet, StandardButtonSet};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    pub delta: i64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Action {
    pub fn std_buttons(&self) -> StandardButtonSet {
        StandardButtonSet::from_bits(self.z as u32)
    }

    pub fn mania_buttons(&self) -> ManiaButtonSet {
        ManiaButtonSet::from_bits(self.x as u32)
    }
}
