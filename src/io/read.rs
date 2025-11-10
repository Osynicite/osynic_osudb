use chrono::{DateTime, Duration, TimeZone, Utc};
use nom::{
    Err as NomErr, IResult, Parser,
    bytes::complete::{tag, take, take_while, take_while1},
    combinator::{cond, map, map_opt, map_res, opt},
    error::{Error as NomError, ErrorKind as NomErrorKind},
    multi::{length_count, length_data, many0},
};
use std::convert::identity;
use std::path::Path;

use crate::entity::collection::collection::Collection;
use crate::entity::collection::collectiondb::CollectionDB;
use crate::entity::osu::beatmap::Beatmap;
use crate::entity::osu::field::grade::Grade;
use crate::entity::osu::field::mode::Mode;
use crate::entity::osu::field::modification::ModSet;
use crate::entity::osu::field::rank::RankedStatus;
use crate::entity::osu::field::time::TimingPoint;
use crate::entity::osu::osudb::OsuDB;
use crate::entity::scores::field::action::Action;
use crate::entity::scores::field::replay::Replay;
use crate::entity::scores::scores::Scores;
use crate::entity::scores::scoresdb::ScoresDB;
use crate::error::Result;
use crate::io::bit::Bit;
use crate::{CHANGE_20140609, CHANGE_20191106, CHANGE_20250107};

pub use nom::number::complete::le_f32 as float;
pub use nom::number::complete::le_f64 as double;
pub use nom::number::complete::le_u8 as byte;
pub use nom::number::complete::le_u16 as short;
pub use nom::number::complete::le_u32 as int;
pub use nom::number::complete::le_u64 as long;

impl Replay {
    pub fn from_bytes(bytes: &[u8]) -> Result<Replay> {
        replay(bytes, true).map(|(_rem, replay)| replay)
    }
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Replay> {
        Self::from_bytes(&std::fs::read(path)?)
    }
}

impl ScoresDB {
    pub fn from_bytes(bytes: &[u8]) -> Result<ScoresDB> {
        scores(bytes).map(|(_rem, scores)| scores)
    }
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<ScoresDB> {
        Self::from_bytes(&std::fs::read(path)?)
    }
}

impl CollectionDB {
    pub fn from_bytes(bytes: &[u8]) -> Result<CollectionDB> {
        Ok(collections(bytes).map(|(_rem, collections)| collections)?)
    }
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<CollectionDB> {
        Self::from_bytes(&std::fs::read(path)?)
    }
}

impl OsuDB {
    pub fn from_bytes(bytes: &[u8]) -> Result<OsuDB> {
        Ok(osudb(bytes).map(|(_rem, osudb)| osudb)?)
    }
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<OsuDB> {
        Self::from_bytes(&std::fs::read(path)?)
    }
}

pub fn scores(bytes: &[u8]) -> Result<(&[u8], ScoresDB)> {
    let (rem, version) = int(bytes)?;
    let (mut rem, len) = int(rem)?;
    let mut beatmaps = Vec::with_capacity(len as usize);

    for _ in 0..len {
        let (rem_, beatmap_scores) = beatmap_scores(rem)?;
        beatmaps.push(beatmap_scores);
        rem = rem_;
    }

    let list = ScoresDB { version, beatmaps };

    Ok((rem, list))
}

pub fn beatmap_scores(bytes: &[u8]) -> Result<(&[u8], Scores)> {
    let (rem, hash) = opt_string(bytes)?;
    let (mut rem, len) = int(rem)?;
    let mut scores = Vec::with_capacity(len as usize);

    for _ in 0..len {
        let (rem_, replay) = replay(rem, false)?;
        rem = rem_;
        scores.push(replay);
    }

    let scores = Scores { hash, scores };

    Ok((rem, scores))
}

pub fn collections(bytes: &[u8]) -> IResult<&[u8], CollectionDB> {
    let (rem, version) = int(bytes)?;
    let (rem, collections) = length_count(map(int, identity), collection).parse(rem)?;

    let list = CollectionDB {
        version,
        collections,
    };

    Ok((rem, list))
}

pub fn collection(bytes: &[u8]) -> IResult<&[u8], Collection> {
    let (rem, name) = opt_string(bytes)?;
    let (rem, beatmap_hashes) = length_count(map(int, identity), opt_string).parse(rem)?;

    let collection = Collection {
        name,
        beatmap_hashes,
    };

    Ok((rem, collection))
}

pub fn osudb(bytes: &[u8]) -> IResult<&[u8], OsuDB> {
    let (rem, version) = int(bytes)?;
    let (rem, folder_count) = int(rem)?;
    let (rem, account_unlocked) = boolean(rem)?;
    let (rem, unlock_date) = datetime(rem)?;
    let (rem, player_name) = opt_string(rem)?;
    let (rem, beatmaps) =
        length_count(map(int, identity), |bytes| beatmap(bytes, version)).parse(rem)?;
    let (rem, user_permissions) = int(rem)?;

    let osudb = OsuDB {
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
    let (rem, _beatmap_size) = cond(version < CHANGE_20191106, int).parse(bytes)?;
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
    let (rem, timing_points) = length_count(map(int, identity), timing_point).parse(rem)?;
    let (rem, beatmap_id) = int(rem)?;
    let (rem, beatmapset_id) = int(rem)?;
    let (rem, thread_id) = int(rem)?;
    let (rem, std_grade) = grade(rem)?;
    let (rem, taiko_grade) = grade(rem)?;
    let (rem, ctb_grade) = grade(rem)?;
    let (rem, mania_grade) = grade(rem)?;
    let (rem, local_beatmap_offset) = short(rem)?;
    let (rem, stack_leniency) = float(rem)?;
    let (rem, mode) = map_opt(byte, Mode::from_raw).parse(rem)?;
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
    let (rem, mysterious_short) = cond(version < CHANGE_20140609, short).parse(rem)?;
    let (rem, mysterious_last_modified) = int(rem)?;
    let (rem, mania_scroll_speed) = byte(rem)?;

    let beatmap = Beatmap {
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

    Ok((rem, beatmap))
}

pub fn windows_ticks_to_datetime(ticks: u64) -> DateTime<Utc> {
    let epoch = Utc.with_ymd_and_hms(1, 1, 1, 0, 0, 0).unwrap();
    epoch
        + Duration::microseconds((ticks / 10) as i64)
        + Duration::nanoseconds((ticks % 10 * 100) as i64)
}

pub fn datetime(bytes: &[u8]) -> IResult<&[u8], DateTime<Utc>> {
    map(long, windows_ticks_to_datetime).parse(bytes)
}

pub fn datetime_to_windows_ticks(datetime: &DateTime<Utc>) -> u64 {
    let epoch = Utc.with_ymd_and_hms(1, 1, 1, 0, 0, 0).unwrap();
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
            let (rem, string) = map_res(take(len), std::str::from_utf8).parse(rem)?;

            Ok((rem, Some(string.to_owned())))
        }
        _ => Err(NomErr::Error(NomError::new(bytes, NomErrorKind::Switch))),
    }
}

pub fn boolean(bytes: &[u8]) -> IResult<&[u8], bool> {
    map(byte, |byte: u8| byte != 0).parse(bytes)
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
        length_count(map(int, identity), |bytes| star_rating(bytes, version)).parse(bytes)
    } else {
        Ok((bytes, Vec::new()))
    }
}

pub fn star_rating(bytes: &[u8], version: u32) -> IResult<&[u8], (ModSet, f64)> {
    let (rem, _tag) = tag(&[0x08][..])(bytes)?;
    let (rem, mods) = map(int, ModSet::from_bits).parse(rem)?;

    if version < CHANGE_20250107 {
        let (rem, _tag) = tag(&[0x0d][..])(rem)?;
        let (rem, stars) = double(rem)?;
        Ok((rem, (mods, stars)))
    } else {
        let (rem, _tag) = tag(&[0x0c][..])(rem)?;
        let (rem, stars) = float(rem)?;
        Ok((rem, (mods, stars as f64)))
    }
}

pub fn difficulty_value(bytes: &[u8], version: u32) -> IResult<&[u8], f32> {
    if version >= CHANGE_20140609 {
        float(bytes)
    } else {
        byte(bytes).map(|(rem, b)| (rem, b as f32))
    }
}

pub fn ranked_status(bytes: &[u8]) -> IResult<&[u8], RankedStatus> {
    map_opt(byte, RankedStatus::from_raw).parse(bytes)
}

pub fn grade(bytes: &[u8]) -> IResult<&[u8], Grade> {
    map_opt(byte, Grade::from_raw).parse(bytes)
}

fn parse_replay_data(raw: Option<&[u8]>) -> Result<Option<Vec<Action>>> {
    if let Some(raw) = raw {
        use liblzma::read::XzDecoder;
        use std::io::Read;

        let mut decoder = XzDecoder::new(&raw[..]);
        let mut data = Vec::new();
        decoder.read_to_end(&mut data)?;
        
        let actions = actions(&data)?.1;
        return Ok(Some(actions));
    }
    Ok(None)
}

pub(crate) fn replay(bytes: &[u8], standalone: bool) -> Result<(&[u8], Replay)> {
    let (rem, mode) = map_opt(byte, Mode::from_raw).parse(bytes)?;
    let (rem, version) = int(rem)?;
    let (rem, beatmap_hash) = opt_string(rem)?;
    let (rem, player_name) = opt_string(rem)?;
    let (rem, replay_hash) = opt_string(rem)?;
    let (rem, count_300) = short(rem)?;
    let (rem, count_100) = short(rem)?;
    let (rem, count_50) = short(rem)?;
    let (rem, count_geki) = short(rem)?;
    let (rem, count_katsu) = short(rem)?;
    let (rem, count_miss) = short(rem)?;
    let (rem, score) = int(rem)?;
    let (rem, max_combo) = short(rem)?;
    let (rem, perfect_combo) = boolean(rem)?;
    let (rem, mods) = map(int, ModSet::from_bits).parse(rem)?;
    let (rem, life_graph) = opt_string(rem)?;
    let (rem, timestamp) = datetime(rem)?;

    let (rem, raw_replay_data) = if standalone {
        map(length_data(int), Some).parse(rem)?
    } else {
        let (rem, _tag) = tag(&[0xff, 0xff, 0xff, 0xff][..])(rem)?;

        (rem, None)
    };

    let replay_data = parse_replay_data(raw_replay_data)?;
    let (rem, online_score_id) = long(rem)?;

    let replay = Replay {
        mode,
        version,
        beatmap_hash,
        player_name,
        replay_hash,
        count_300,
        count_100,
        count_50,
        count_geki,
        count_katsu,
        count_miss,
        score,
        max_combo,
        perfect_combo,
        mods,
        life_graph,
        timestamp,
        replay_data,
        raw_replay_data: raw_replay_data.map(ToOwned::to_owned),
        online_score_id,
    };

    Ok((rem, replay))
}
pub fn actions(bytes: &[u8]) -> IResult<&[u8], Vec<Action>> {
    many0(action).parse(bytes)
}

pub fn action(bytes: &[u8]) -> IResult<&[u8], Action> {
    let (rem, delta) = number(bytes)?;
    let (rem, _tag) = tag(&b"|"[..])(rem)?;
    let (rem, x) = number(rem)?;
    let (rem, _tag) = tag(&b"|"[..])(rem)?;
    let (rem, y) = number(rem)?;
    let (rem, _tag) = tag(&b"|"[..])(rem)?;
    let (rem, z) = number(rem)?;
    let (rem, _tag) = tag(&b","[..])(rem)?;

    let action = Action {
        delta: delta as i64,
        x: x as f32,
        y: y as f32,
        z: z as f32,
    };

    Ok((rem, action))
}
pub fn number(bytes: &[u8]) -> IResult<&[u8], f64> {
    let (rem, sign) = opt(tag(&b"-"[..])).parse(bytes)?;
    let (rem, whole) = take_while1(|b: u8| b.is_ascii_digit())(rem)?;
    let (rem, decimal) = opt(number_bytes).parse(rem)?;

    let mut num = 0.0;

    for byte in whole {
        num *= 10.0;
        num += (*byte - b'0') as f64;
    }

    if let Some(decimal) = decimal {
        let mut value = 1.0;

        for byte in decimal {
            value /= 10.0;
            num += (*byte - b'0') as f64 * value;
        }
    }

    if sign.is_some() {
        num *= -1.0
    }

    Ok((rem, num))
}

pub fn number_bytes(bytes: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rem, _tag) = tag(&b"."[..])(bytes)?;

    take_while(|b: u8| b.is_ascii_digit())(rem)
}

fn build_option<T>(is_none: bool, content: T) -> Option<T> {
    if is_none { None } else { Some(content) }
}
