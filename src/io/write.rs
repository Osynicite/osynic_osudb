use crate::error::Result;
use std::io::Write;
use chrono::{DateTime,Utc};

use crate::{CHANGE_20140609, CHANGE_20191106};
use crate::io::read::*;
use crate::io::bit::Bit;
use crate::entity::osudb::Osudb;
use crate::entity::beatmap::Beatmap;
use crate::entity::field::rank::RankedStatus;
use crate::entity::field::time::TimingPoint;
use crate::entity::field::grade::Grade;
use crate::entity::field::modification::ModSet;

pub trait Writable {
    type Args;
    fn wr_args<W: Write>(&self, out: &mut W, args: Self::Args) -> Result<()>;
}
pub trait SimpleWritable
where
    Self: Writable,
{
    fn wr<W: Write>(&self, out: &mut W) -> Result<()>;
}
impl<T> SimpleWritable for T
where
    T: Writable<Args = ()>,
{
    fn wr<W: Write>(&self, out: &mut W) -> Result<()> {
        self.wr_args(out, ())
    }
}

struct PrefixedList<'a, T>(&'a [T]);
impl<T> Writable for PrefixedList<'_, T>
where
    T: Writable,
    T::Args: Clone,
{
    type Args = T::Args;
    fn wr_args<W: Write>(&self, out: &mut W, args: T::Args) -> Result<()> {
        (self.0.len() as u32).wr(out)?;
        for item in self.0 {
            item.wr_args(out, args.clone())?;
        }
        Ok(())
    }
}

macro_rules! writer {
    ($type:ty [$this:ident, $out:ident] $code:expr) => {
        writer!($type [$this, $out, _arg: ()] $code);
    };
    ($type:ty [$this:ident, $out:ident, $args:ident : $args_ty:ty] $code:expr) => {
        impl Writable for $type {
            type Args=$args_ty;
            fn wr_args<W: Write>(&self, $out: &mut W, $args: $args_ty) -> Result<()> {
                let $this = self;
                let () = $code;
                Ok(())
            }
        }
    };
}

writer!(u8 [this,out] out.write_all(&this.to_le_bytes())?);
writer!(u16 [this,out] out.write_all(&this.to_le_bytes())?);
writer!(u32 [this,out] out.write_all(&this.to_le_bytes())?);
writer!(u64 [this,out] out.write_all(&this.to_le_bytes())?);
writer!(f32 [this,out] this.to_bits().wr(out)?);
writer!(f64 [this,out] this.to_bits().wr(out)?);
writer!(bool [this,out] (if *this {1_u8} else {0_u8}).wr(out)?);
writer!(DateTime<Utc> [this,out] datetime_to_windows_ticks(this).wr(out)?);
writer!(usize [this,out] {
    let mut this=*this;
    loop {
        let mut byte=this as u8;
        this>>=7;
        let continues={this!=0};
        byte.set_bit(7, continues);
        byte.wr(out)?;
        if !continues {break}
    }
});
writer!(Option<String> [this,out] {
    match this {
        Some(string) => {
            0x0b_u8.wr(out)?;
            string.len().wr(out)?;
            out.write_all(string.as_bytes())?;
        },
        None => 0x00_u8.wr(out)?,
    }
});
writer!(Osudb [this, out] {
    this.version.wr(out)?;
    this.folder_count.wr(out)?;
    write_option(out,this.unban_date,0_u64)?;
    this.player_name.wr(out)?;
    PrefixedList(&this.beatmaps).wr_args(out,this.version)?;
    this.user_permissions.wr(out)?;
});
writer!(Beatmap [this,out,version: u32] {
    fn write_dry<W: Write>(this: &Beatmap, out: &mut W, version: u32) -> Result<()> {
        macro_rules! wr_difficulty_value {
            ($f32:expr) => {{
                if version>=CHANGE_20140609 {
                    $f32.wr(out)?;
                }else{
                    ($f32 as u8).wr(out)?;
                }
            }};
        }
        this.artist_ascii.wr(out)?;
        this.artist_unicode.wr(out)?;
        this.title_ascii.wr(out)?;
        this.title_unicode.wr(out)?;
        this.creator.wr(out)?;
        this.difficulty_name.wr(out)?;
        this.audio.wr(out)?;
        this.hash.wr(out)?;
        this.file_name.wr(out)?;
        this.status.wr(out)?;
        this.hitcircle_count.wr(out)?;
        this.slider_count.wr(out)?;
        this.spinner_count.wr(out)?;
        this.last_modified.wr(out)?;
        wr_difficulty_value!(this.approach_rate);
        wr_difficulty_value!(this.circle_size);
        wr_difficulty_value!(this.hp_drain);
        wr_difficulty_value!(this.overall_difficulty);
        this.slider_velocity.wr(out)?;
        this.std_ratings.wr_args(out,version)?;
        this.taiko_ratings.wr_args(out,version)?;
        this.ctb_ratings.wr_args(out,version)?;
        this.mania_ratings.wr_args(out,version)?;
        this.drain_time.wr(out)?;
        this.total_time.wr(out)?;
        this.preview_time.wr(out)?;
        PrefixedList(&this.timing_points).wr(out)?;
        (this.beatmap_id as u32).wr(out)?;
        (this.beatmapset_id as u32).wr(out)?;
        this.thread_id.wr(out)?;
        this.std_grade.wr(out)?;
        this.taiko_grade.wr(out)?;
        this.ctb_grade.wr(out)?;
        this.mania_grade.wr(out)?;
        this.local_beatmap_offset.wr(out)?;
        this.stack_leniency.wr(out)?;
        this.mode.raw().wr(out)?;
        this.song_source.wr(out)?;
        this.tags.wr(out)?;
        this.online_offset.wr(out)?;
        this.title_font.wr(out)?;
        write_option(out,this.last_played,0_u64)?;
        this.is_osz2.wr(out)?;
        this.folder_name.wr(out)?;
        this.last_online_check.wr(out)?;
        this.ignore_sounds.wr(out)?;
        this.ignore_skin.wr(out)?;
        this.disable_storyboard.wr(out)?;
        this.disable_video.wr(out)?;
        this.visual_override.wr(out)?;
        if version<CHANGE_20140609 {
            this.mysterious_short.unwrap_or(0).wr(out)?;
        }
        this.mysterious_last_modified.wr(out)?;
        this.mania_scroll_speed.wr(out)?;
        Ok(())
    }
    if version < CHANGE_20191106 {
        let mut raw_buf = Vec::new();
        write_dry(this, &mut raw_buf, version)?;
        (raw_buf.len() as u32).wr(out)?;
        out.write_all(&raw_buf)?;
    }else{
        write_dry(this, out, version)?;
    }
});

writer!(TimingPoint [this,out] {
    this.bpm.wr(out)?;
    this.offset.wr(out)?;
    this.inherits.wr(out)?;
});

writer!(Vec<(ModSet,f64)> [this,out,version: u32] {
    if version>=CHANGE_20140609 {
        PrefixedList(this).wr(out)?;
    }
});
writer!((ModSet,f64) [this,out] {
    0x08_u8.wr(out)?;
    this.0.bits().wr(out)?;
    0x0d_u8.wr(out)?;
    this.1.wr(out)?;
});

writer!(RankedStatus [this,out] this.raw().wr(out)?);

writer!(Grade [this,out] this.raw().wr(out)?);


fn write_option<W: Write, T: SimpleWritable, D: SimpleWritable>(
    out: &mut W,
    opt: Option<T>,
    def: D,
) -> Result<()> {
    match opt {
        Some(t) => {
            false.wr(out)?;
            t.wr(out)?;
        }
        None => {
            true.wr(out)?;
            def.wr(out)?;
        }
    }
    Ok(())
}
