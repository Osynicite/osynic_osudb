use nom::{
    bytes::complete::{tag, take, take_while},
    combinator::{cond, map, map_opt, map_res},
    error::{Error as NomError, ErrorKind as NomErrorKind},
    multi::length_count,
    Err as NomErr, IResult
};
use std::convert::identity;
use chrono::{DateTime, Duration, TimeZone, Utc};

use crate::{CHANGE_20140609, CHANGE_20191106};
use crate::io::bit::Bit;
use crate::entity::osudb::Osudb;
use crate::entity::beatmap::Beatmap;
use crate::entity::field::rank::RankedStatus;
use crate::entity::field::time::TimingPoint;
use crate::entity::field::grade::Grade;
use crate::entity::field::modification::ModSet;
use crate::entity::field::mode::Mode;

pub use nom::number::complete::le_f32 as single;
pub use nom::number::complete::le_f64 as double;
pub use nom::number::complete::le_u16 as short;
pub use nom::number::complete::le_u32 as int;
pub use nom::number::complete::le_u64 as long;
pub use nom::number::complete::le_u8 as byte;



pub fn osudb(bytes: &[u8]) -> IResult<&[u8], Osudb> {
    let (rem, version) = int(bytes)?;
    let (rem, folder_count) = int(rem)?;
    let (rem, account_unlocked) = boolean(rem)?;
    let (rem, unlock_date) = datetime(rem)?;
    let (rem, player_name) = opt_string(rem)?;
    let (rem, beatmaps) = length_count(map(int, identity), |bytes| beatmap(bytes, version))(rem)?;
    let (rem, user_permissions) = int(rem)?;

    let osudb = Osudb {
        version,
        folder_count,
        unban_date: build_option(account_unlocked, unlock_date),
        player_name,
        beatmaps,
        user_permissions,
    };

    Ok((rem, osudb))
}

pub fn beatmap(bytes: &[u8], version: u32) -> IResult<&[u8], Beatmap> {
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




pub fn windows_ticks_to_datetime(ticks: u64) -> DateTime<Utc> {
    let epoch = Utc.with_ymd_and_hms(1, 1, 1,0, 0, 0).unwrap();
    epoch
        + Duration::microseconds((ticks / 10) as i64)
        + Duration::nanoseconds((ticks % 10 * 100) as i64)
}

pub fn datetime(bytes: &[u8]) -> IResult<&[u8], DateTime<Utc>> {
    map(long, windows_ticks_to_datetime)(bytes)
}

pub fn datetime_to_windows_ticks(datetime: &DateTime<Utc>) -> u64 {
    let epoch = Utc.with_ymd_and_hms(1, 1, 1,0, 0, 0).unwrap();
    let duration = datetime.signed_duration_since(epoch);
    let ticks_since: i64 = (duration * 10).num_microseconds().unwrap_or(0);
    ticks_since.max(0) as u64
}


pub fn uleb(bytes: &[u8]) -> IResult<&[u8], usize> {
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

pub fn opt_string(bytes: &[u8]) -> IResult<&[u8], Option<String>> {
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

pub fn boolean(bytes: &[u8]) -> IResult<&[u8], bool> {
    map(byte, |byte: u8| byte != 0)(bytes)
}

pub fn timing_point(bytes: &[u8]) -> IResult<&[u8], TimingPoint> {
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

pub fn star_ratings(bytes: &[u8], version: u32) -> IResult<&[u8], Vec<(ModSet, f64)>> {
    if version >= CHANGE_20140609 {
        length_count(map(int, identity), star_rating)(bytes)
    } else {
        Ok((bytes, Vec::new()))
    }
}

pub fn star_rating(bytes: &[u8]) -> IResult<&[u8], (ModSet, f64)> {
    let (rem, _tag) = tag(&[0x08])(bytes)?;
    let (rem, mods) = map(int, ModSet::from_bits)(rem)?;
    let (rem, _tag) = tag(&[0x0d])(rem)?;
    let (rem, stars) = double(rem)?;

    Ok((rem, (mods, stars)))
}

pub fn difficulty_value(bytes: &[u8], version: u32) -> IResult<&[u8], f32> {
    if version >= CHANGE_20140609 {
        single(bytes)
    } else {
        byte(bytes).map(|(rem, b)| (rem, b as f32))
    }
}

pub fn ranked_status(bytes: &[u8]) -> IResult<&[u8], RankedStatus> {
    map_opt(byte, RankedStatus::from_raw)(bytes)
}

pub fn grade(bytes: &[u8]) -> IResult<&[u8], Grade> {
    map_opt(byte, Grade::from_raw)(bytes)
}

fn build_option<T>(is_none: bool, content: T) -> Option<T> {
    if is_none {
        None
    } else {
        Some(content)
    }
}