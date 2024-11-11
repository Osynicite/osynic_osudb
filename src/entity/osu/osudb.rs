use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::entity::osu::beatmap::Beatmap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OsuDB {
    pub version: u32,
    pub folder_count: u32,
    pub unban_date: Option<DateTime<Utc>>,
    pub player_name: Option<String>,
    pub beatmaps: Vec<Beatmap>,
    pub user_permissions: u32,
}
