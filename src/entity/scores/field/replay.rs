use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::entity::osu::field::mode::Mode;
use crate::entity::osu::field::modification::ModSet;
use crate::entity::scores::field::action::Action;

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Replay {
    pub mode: Mode,
    pub version: u32,
    pub beatmap_hash: Option<String>,
    pub player_name: Option<String>,
    pub replay_hash: Option<String>,
    pub count_300: u16,
    pub count_100: u16,
    pub count_50: u16,
    pub count_geki: u16,
    pub count_katsu: u16,
    pub count_miss: u16,
    pub score: u32,
    pub max_combo: u16,
    pub perfect_combo: bool,
    pub mods: ModSet,
    pub life_graph: Option<String>,
    #[cfg_attr(feature = "export", tsify(type = "Date"))]
    pub timestamp: DateTime<Utc>,
    pub replay_data: Option<Vec<Action>>,
    pub raw_replay_data: Option<Vec<u8>>,
    pub online_score_id: u64,
}
