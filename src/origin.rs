#![allow(unused_imports)]
#![allow(dead_code)]
const CHANGE_20140609: u32 = 20140609;
const CHANGE_20191106: u32 = 20191106;

pub use crate::error::{Result, Error};

use std::{
    fmt,
    fs::{self, File},
    ops,
    path::Path,convert::identity, hash::Hash
};
use nom::{
    bytes::complete::{tag, take, take_while, take_while1},
    combinator::{cond, map, map_opt, map_res, opt},
    error::{Error as NomError, ErrorKind as NomErrorKind},
    multi::{length_count, length_data, many0},
    Err as NomErr, IResult, Needed,
};
use chrono::{DateTime, Duration, TimeZone, Utc};
use serde::{Deserialize, Serialize};

trait Bit {
    fn bit(&self, pos: u32) -> bool;
    fn bit_range(&self, pos: std::ops::Range<u32>) -> Self;
    fn set_bit(&mut self, pos: u32, val: bool);
    fn set_bit_range(&mut self, pos: std::ops::Range<u32>, val: Self);
}

macro_rules! impl_bit {
    (@ $ty:ty) => {
        impl Bit for $ty {
            fn bit(&self, pos: u32) -> bool {
                (*self & 1 << pos) != 0
            }
            fn bit_range(&self, pos: ops::Range<u32>) -> Self {
                (*self & ((1<<pos.end)-1)) >> pos.start
            }
            fn set_bit(&mut self, pos: u32, val: bool) {
                *self = (*self & !(1<<pos)) | ((val as Self)<<pos);
            }
            fn set_bit_range(&mut self, pos: ops::Range<u32>, val: Self) {
                let mask = ((1<<(pos.end-pos.start))-1) << pos.start;
                *self = (*self & !mask) | ((val<<pos.start)&mask);
            }
        }
    };
    ($($ty:ty),*) => {
        $(
            impl_bit!(@ $ty);
        )*
    }
}

impl_bit!(u8, u16, u32, u64);

// impl Bit for u8 {
//     fn bit(&self, pos: u32) -> bool {
//         (*self & (1 << pos)) != 0
//     }
//     fn bit_range(&self, pos: std::ops::Range<u32>) -> Self {
//         (*self & ((1 << pos.end) - 1)) >> pos.start
//     }
//     fn set_bit(&mut self, pos: u32, val: bool) {
//         *self = (*self & !(1 << pos)) | ((val as Self) << pos);
//     }
//     fn set_bit_range(&mut self, pos: std::ops::Range<u32>, val: Self) {
//         let mask = ((1 << (pos.end - pos.start)) - 1) << pos.start;
//         *self = (*self & !mask) | ((val << pos.start) & mask);
//     }
// }

//Common fixed-size osu `.db` primitives.
use nom::number::complete::le_f32 as single;
use nom::number::complete::le_f64 as double;
use nom::number::complete::le_u16 as short;
use nom::number::complete::le_u32 as int;
use nom::number::complete::le_u64 as long;
use nom::number::complete::le_u8 as byte;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Osudb {
    pub version: u32,
    pub folder_count: u32,
    pub unban_date: Option<DateTime<Utc>>,
    pub player_name: Option<String>,
    pub beatmaps: Vec<Beatmap>,
    pub user_permissions: u32,
}
impl Osudb {
    pub fn from_bytes(bytes: &[u8]) -> Result<Osudb> {
        Ok(listing(bytes).map(|(_rem, listing)| listing)?)
    }
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Osudb> {
        Self::from_bytes(&fs::read(path)?)
    }
}

// TODO Deprecated methods ymd and hms, use with_ymd_and_hms instead. But the Result need to be solved.
/// Get a datetime from an amount of "windows ticks":
/// The amount of 100-nanosecond units since midnight of the date 0001/01/01.
fn windows_ticks_to_datetime(ticks: u64) -> DateTime<Utc> {
    let epoch = Utc.ymd(1, 1, 1).and_hms(0, 0, 0);
    epoch
        + Duration::microseconds((ticks / 10) as i64)
        + Duration::nanoseconds((ticks % 10 * 100) as i64)
}

fn datetime(bytes: &[u8]) -> IResult<&[u8], DateTime<Utc>> {
    map(long, windows_ticks_to_datetime)(bytes)
}

fn datetime_to_windows_ticks(datetime: &DateTime<Utc>) -> u64 {
    let epoch = Utc.ymd(1, 1, 1).and_hms(0, 0, 0);
    let duration = datetime.signed_duration_since(epoch);
    let ticks_since: i64 = (duration * 10).num_microseconds().unwrap_or(0);
    ticks_since.max(0) as u64
}

// The variable-length ULEB128 encoding used mainly for string lengths.
fn uleb(bytes: &[u8]) -> IResult<&[u8], usize> {
    let (rem, prelude) = take_while(|b: u8| b.bit(7))(bytes)?;
    let (rem, finalizer) = byte(rem)?;

    let mut out = 0;
    let mut offset = 0;

    for byte in prelude {
        out |= (byte.bit_range(0..7) as usize) << offset;
        offset += 7;
    }

    out |= (finalizer as usize) << offset;

    Ok((rem, out))
}

fn boolean(bytes: &[u8]) -> IResult<&[u8], bool> {
    map(byte, |byte: u8| byte != 0)(bytes)
}

fn opt_string(bytes: &[u8]) -> IResult<&[u8], Option<String>> {
    let (rem, first_byte) = byte(bytes)?;

    match first_byte {
        0x00 => Ok((rem, None)),
        0x0b => {
            let (rem, len) = uleb(rem)?;
            let (rem, string) = map_res(take(len), std::str::from_utf8)(rem)?;

            Ok((rem, Some(string.to_owned())))
        }
        _ => Err(NomErr::Error(NomError::new(bytes, NomErrorKind::Switch))),
    }
}

fn build_option<T>(is_none: bool, content: T) -> Option<T> {
    if is_none {
        None
    } else {
        Some(content)
    }
}

/// Before the breaking change in 2014 several difficulty values were stored as bytes.
/// After it they were stored as single floats.
/// Accomodate this differences.
fn difficulty_value(bytes: &[u8], version: u32) -> IResult<&[u8], f32> {
    if version >= CHANGE_20140609 {
        single(bytes)
    } else {
        byte(bytes).map(|(rem, b)| (rem, b as f32))
    }
}

fn ranked_status(bytes: &[u8]) -> IResult<&[u8], RankedStatus> {
    map_opt(byte, RankedStatus::from_raw)(bytes)
}

fn grade(bytes: &[u8]) -> IResult<&[u8], Grade> {
    map_opt(byte, Grade::from_raw)(bytes)
}

fn star_ratings(bytes: &[u8], version: u32) -> IResult<&[u8], Vec<(ModSet, f64)>> {
    if version >= CHANGE_20140609 {
        length_count(map(int, identity), star_rating)(bytes)
    } else {
        Ok((bytes, Vec::new()))
    }
}

fn star_rating(bytes: &[u8]) -> IResult<&[u8], (ModSet, f64)> {
    let (rem, _tag) = tag(&[0x08])(bytes)?;
    let (rem, mods) = map(int, ModSet::from_bits)(rem)?;
    let (rem, _tag) = tag(&[0x0d])(rem)?;
    let (rem, stars) = double(rem)?;

    Ok((rem, (mods, stars)))
}

fn timing_point(bytes: &[u8]) -> IResult<&[u8], TimingPoint> {
    let (rem, bpm) = double(bytes)?;
    let (rem, offset) = double(rem)?;
    let (rem, inherits) = boolean(rem)?;

    let timing_point = TimingPoint {
        bpm,
        offset,
        inherits,
    };

    Ok((rem, timing_point))
}


fn beatmap(bytes: &[u8], version: u32) -> IResult<&[u8], Beatmap> {
    let (rem, _beatmap_size) = cond(version < CHANGE_20191106, int)(bytes)?;
    let (rem, artist_ascii) = opt_string(rem)?;
    let (rem, artist_unicode) = opt_string(rem)?;
    let (rem, title_ascii) = opt_string(rem)?;
    let (rem, title_unicode) = opt_string(rem)?;
    let (rem, creator) = opt_string(rem)?;
    let (rem, difficulty_name) = opt_string(rem)?;
    let (rem, audio) = opt_string(rem)?;
    let (rem, hash) = opt_string(rem)?;
    let (rem, file_name) = opt_string(rem)?;
    let (rem, status) = ranked_status(rem)?;
    let (rem, hitcircle_count) = short(rem)?;
    let (rem, slider_count) = short(rem)?;
    let (rem, spinner_count) = short(rem)?;
    let (rem, last_modified) = datetime(rem)?;
    let (rem, approach_rate) = difficulty_value(rem, version)?;
    let (rem, circle_size) = difficulty_value(rem, version)?;
    let (rem, hp_drain) = difficulty_value(rem, version)?;
    let (rem, overall_difficulty) = difficulty_value(rem, version)?;
    let (rem, slider_velocity) = double(rem)?;
    let (rem, std_ratings) = star_ratings(rem, version)?;
    let (rem, taiko_ratings) = star_ratings(rem, version)?;
    let (rem, ctb_ratings) = star_ratings(rem, version)?;
    let (rem, mania_ratings) = star_ratings(rem, version)?;
    let (rem, drain_time) = int(rem)?;
    let (rem, total_time) = int(rem)?;
    let (rem, preview_time) = int(rem)?;
    let (rem, timing_points) = length_count(map(int, identity), timing_point)(rem)?;
    let (rem, beatmap_id) = int(rem)?;
    let (rem, beatmapset_id) = int(rem)?;
    let (rem, thread_id) = int(rem)?;
    let (rem, std_grade) = grade(rem)?;
    let (rem, taiko_grade) = grade(rem)?;
    let (rem, ctb_grade) = grade(rem)?;
    let (rem, mania_grade) = grade(rem)?;
    let (rem, local_beatmap_offset) = short(rem)?;
    let (rem, stack_leniency) = single(rem)?;
    let (rem, mode) = map_opt(byte, Mode::from_raw)(rem)?;
    let (rem, song_source) = opt_string(rem)?;
    let (rem, tags) = opt_string(rem)?;
    let (rem, online_offset) = short(rem)?;
    let (rem, title_font) = opt_string(rem)?;
    let (rem, unplayed) = boolean(rem)?;
    let (rem, last_played) = datetime(rem)?;
    let (rem, is_osz2) = boolean(rem)?;
    let (rem, folder_name) = opt_string(rem)?;
    let (rem, last_online_check) = datetime(rem)?;
    let (rem, ignore_sounds) = boolean(rem)?;
    let (rem, ignore_skin) = boolean(rem)?;
    let (rem, disable_storyboard) = boolean(rem)?;
    let (rem, disable_video) = boolean(rem)?;
    let (rem, visual_override) = boolean(rem)?;
    let (rem, mysterious_short) = cond(version < CHANGE_20140609, short)(rem)?;
    let (rem, mysterious_last_modified) = int(rem)?;
    let (rem, mania_scroll_speed) = byte(rem)?;

    let map = Beatmap {
        artist_ascii,
        artist_unicode,
        title_ascii,
        title_unicode,
        creator,
        difficulty_name,
        audio,
        hash,
        file_name,
        status,
        hitcircle_count,
        slider_count,
        spinner_count,
        last_modified,
        approach_rate,
        circle_size,
        hp_drain,
        overall_difficulty,
        slider_velocity,
        std_ratings,
        taiko_ratings,
        ctb_ratings,
        mania_ratings,
        drain_time,
        total_time,
        preview_time,
        timing_points,
        beatmap_id: beatmap_id as i32,
        beatmapset_id: beatmapset_id as i32,
        thread_id,
        std_grade,
        taiko_grade,
        ctb_grade,
        mania_grade,
        local_beatmap_offset,
        stack_leniency,
        mode,
        song_source,
        tags,
        online_offset,
        title_font,
        last_played: build_option(unplayed, last_played),
        is_osz2,
        folder_name,
        last_online_check,
        ignore_sounds,
        ignore_skin,
        disable_storyboard,
        disable_video,
        visual_override,
        mysterious_short,
        mysterious_last_modified,
        mania_scroll_speed,
    };

    Ok((rem, map))
}


fn listing(bytes: &[u8]) -> IResult<&[u8], Osudb> {
    let (rem, version) = int(bytes)?;
    let (rem, folder_count) = int(rem)?;
    let (rem, account_unlocked) = boolean(rem)?;
    let (rem, unlock_date) = datetime(rem)?;
    let (rem, player_name) = opt_string(rem)?;
    let (rem, beatmaps) = length_count(map(int, identity), |bytes| beatmap(bytes, version))(rem)?;
    let (rem, user_permissions) = int(rem)?;

    let listing = Osudb {
        version,
        folder_count,
        unban_date: build_option(account_unlocked, unlock_date),
        player_name,
        beatmaps,
        user_permissions,
    };

    Ok((rem, listing))
}


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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RankedStatus {
    Unknown,
    Unsubmitted,
    PendingWipGraveyard,
    Ranked,
    Approved,
    Qualified,
    Loved,
}
impl RankedStatus {
    pub fn from_raw(byte: u8) -> Option<RankedStatus> {
        use self::RankedStatus::*;
        Some(match byte {
            0 => Unknown,
            1 => Unsubmitted,
            2 => PendingWipGraveyard,
            4 => Ranked,
            5 => Approved,
            6 => Qualified,
            7 => Loved,
            _ => return None,
        })
    }

    pub fn raw(self) -> u8 {
        use self::RankedStatus::*;
        match self {
            Unknown => 0,
            Unsubmitted => 1,
            PendingWipGraveyard => 2,
            Ranked => 4,
            Approved => 5,
            Qualified => 6,
            Loved => 7,
        }
    }
}

pub type StarRatings = Vec<(ModSet, f64)>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimingPoint {
    pub bpm: f64,
    pub offset: f64,
    pub inherits: bool,
}

/// TODO: osu-db: Figure out grades.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Grade {
    SSPlus,
    SPlus,
    SS,
    S,
    A,
    B,
    C,
    D,
    Unplayed,
}
impl Grade {
    pub fn raw(self) -> u8 {
        use self::Grade::*;
        match self {
            SSPlus => 0,
            SPlus => 1,
            SS => 2,
            S => 3,
            A => 4,
            B => 5,
            C => 6,
            D => 7,
            Unplayed => 9,
        }
    }
    pub fn from_raw(raw: u8) -> Option<Grade> {
        use self::Grade::*;
        Some(match raw {
            0 => SSPlus,
            1 => SPlus,
            2 => SS,
            3 => S,
            4 => A,
            5 => B,
            6 => C,
            7 => D,
            9 => Unplayed,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Mode {
    Standard,
    Taiko,
    CatchTheBeat,
    Mania,
}
impl Mode {
    pub fn raw(self) -> u8 {
        self as u8
    }

    pub fn from_raw(raw: u8) -> Option<Mode> {
        use self::Mode::*;
        Some(match raw {
            0 => Standard,
            1 => Taiko,
            2 => CatchTheBeat,
            3 => Mania,
            _ => return None,
        })
    }
}

/// A single osu! mod.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash,Serialize, Deserialize)]
#[repr(u8)]
pub enum Mod {
    NoFail,
    Easy,
    TouchDevice,
    Hidden,
    HardRock,
    SuddenDeath,
    DoubleTime,
    Relax,
    HalfTime,
    /// Always goes with `DoubleTime`.
    Nightcore,
    Flashlight,
    Autoplay,
    SpunOut,
    /// Also called "Relax2".
    Autopilot,
    Perfect,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    FadeIn,
    Random,
    /// Cinema.
    LastMod,
    /// Only on osu!cuttingedge it seems.
    TargetPractice,
    Key9,
    Coop,
    Key1,
    Key3,
    Key2,
}
impl Mod {
    /// Each of the 29 mods have a corresponding integer between [0,28], inclusive.
    /// This method retrieves its integer.
    pub fn raw(&self) -> u8 {
        *self as u8
    }

    /// Build a mod from its corresponding integer.
    /// Returns `None` if the integer is out-of-range (>28).
    pub fn from_raw(bit_offset: u8) -> Option<Mod> {
        use self::Mod::*;
        Some(match bit_offset {
            0 => NoFail,
            1 => Easy,
            2 => TouchDevice,
            3 => Hidden,
            4 => HardRock,
            5 => SuddenDeath,
            6 => DoubleTime,
            7 => Relax,
            8 => HalfTime,
            9 => Nightcore,
            10 => Flashlight,
            11 => Autoplay,
            12 => SpunOut,
            13 => Autopilot,
            14 => Perfect,
            15 => Key4,
            16 => Key5,
            17 => Key6,
            18 => Key7,
            19 => Key8,
            20 => FadeIn,
            21 => Random,
            22 => LastMod,
            23 => TargetPractice,
            24 => Key9,
            25 => Coop,
            26 => Key1,
            27 => Key3,
            28 => Key2,
            _ => return None,
        })
    }
}

/// A combination of `Mod`s.
///
/// Very cheap to copy around, as it is a just a wrapped 32-bit integer.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash,Serialize, Deserialize)]
pub struct ModSet(pub u32);
impl ModSet {
    pub fn bits(&self) -> u32 {
        self.0
    }
    pub fn from_bits(bits: u32) -> ModSet {
        ModSet(bits)
    }

    /// Create a `ModSet` with no mods included.
    pub fn empty() -> ModSet {
        ModSet::from_bits(0)
    }

    /// Check whether the set contains the given mod.
    pub fn contains(&self, m: Mod) -> bool {
        self.bits().bit(m.raw() as u32)
    }

    /// Make a new set of mods with the given mod included or not included.
    pub fn set(&self, m: Mod, include: bool) -> ModSet {
        let mut bits = self.bits();
        bits.set_bit(m.raw() as u32, include);
        ModSet::from_bits(bits)
    }

    /// Make a new set of mods with the given mod included.
    pub fn with(&self, m: Mod) -> ModSet {
        self.set(m, true)
    }

    /// Make a new set of mods with the given mod removed.
    pub fn without(&self, m: Mod) -> ModSet {
        self.set(m, false)
    }
}

