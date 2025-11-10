use crate::entity::osu::beatmap::Beatmap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OsuDB {
    pub version: u32,
    pub folder_count: u32,
    #[cfg_attr(feature = "export", tsify(optional, type = "Date"))]
    pub unban_date: Option<DateTime<Utc>>,
    pub player_name: Option<String>,
    pub beatmaps: Vec<Beatmap>,
    pub user_permissions: u32,
}
