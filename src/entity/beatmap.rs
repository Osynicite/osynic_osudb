use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::entity::field::rank::RankedStatus;
use crate::entity::field::mode::Mode;
use crate::entity::field::star::StarRatings;
use crate::entity::field::time::TimingPoint;
use crate::entity::field::grade::Grade;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Beatmap {
    pub artist_ascii: Option<String>,
    pub artist_unicode: Option<String>,
    pub title_ascii: Option<String>,
    pub title_unicode: Option<String>,
    pub creator: Option<String>,
    pub difficulty_name: Option<String>,
    pub audio: Option<String>,
    pub hash: Option<String>,
    pub file_name: Option<String>,
    pub status: RankedStatus,
    pub hitcircle_count: u16,
    pub slider_count: u16,
    pub spinner_count: u16,
    pub last_modified: DateTime<Utc>,
    pub approach_rate: f32,
    pub circle_size: f32,
    pub hp_drain: f32,
    pub overall_difficulty: f32,
    pub slider_velocity: f64,
    pub std_ratings: StarRatings,
    pub taiko_ratings: StarRatings,
    pub ctb_ratings: StarRatings,
    pub mania_ratings: StarRatings,
    pub drain_time: u32,
    pub total_time: u32,
    pub preview_time: u32,
    pub timing_points: Vec<TimingPoint>,
    pub beatmap_id: i32,
    pub beatmapset_id: i32,
    pub thread_id: u32,
    pub std_grade: Grade,
    pub taiko_grade: Grade,
    pub ctb_grade: Grade,
    pub mania_grade: Grade,
    pub local_beatmap_offset: u16,
    pub stack_leniency: f32,
    pub mode: Mode,
    pub song_source: Option<String>,
    pub tags: Option<String>,
    pub online_offset: u16,
    pub title_font: Option<String>,
    pub last_played: Option<DateTime<Utc>>,
    pub is_osz2: bool,
    pub folder_name: Option<String>,
    pub last_online_check: DateTime<Utc>,
    pub ignore_sounds: bool,
    pub ignore_skin: bool,
    pub disable_storyboard: bool,
    pub disable_video: bool,
    pub visual_override: bool,
    pub mysterious_short: Option<u16>,
    pub mysterious_last_modified: u32,
    pub mania_scroll_speed: u8,
}
