use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::error::Result;
use crate::entity::beatmap::Beatmap;
use crate::io::read::osudb;
use crate::io::write::SimpleWritable;

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
        Ok(osudb(bytes).map(|(_rem, listing)| listing)?)
    }
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Osudb> {
        Self::from_bytes(&fs::read(path)?)
    }
}

impl Osudb {
    pub fn to_writer<W: Write>(&self, mut out: W) -> Result<()> {
        self.wr(&mut out)
    }
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.to_writer(BufWriter::new(File::create(path)?))
    }
}



