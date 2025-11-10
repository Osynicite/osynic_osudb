#![doc = include_str!("../README_EN.md")]

pub const CHANGE_20140609: u32 = 20140609;
pub const CHANGE_20191106: u32 = 20191106;
pub const CHANGE_20250107: u32 = 20250107;

pub mod entity;
pub mod error;
pub mod io;

#[cfg(all(feature = "wasm", feature = "export"))]
pub mod wasm;

pub mod prelude {
    // 第一层：顶级数据库入口
    pub use crate::entity::collection::collectiondb::CollectionDB;
    pub use crate::entity::osu::osudb::OsuDB;
    pub use crate::entity::scores::scoresdb::ScoresDB;

    // 第二层：高频操作对象
    pub use crate::entity::collection::collection::Collection;
    pub use crate::entity::osu::beatmap::Beatmap;
    pub use crate::entity::scores::scores::Scores;

    // 第三层：常用枚举/配置
    pub use crate::entity::osu::field::grade::Grade;
    pub use crate::entity::osu::field::mode::Mode;
    pub use crate::entity::osu::field::modification::Mod;
    pub use crate::entity::osu::field::rank::RankedStatus;
    pub use crate::entity::osu::field::star::StarRatings;

    pub use crate::entity::scores::field::action::Action;
    pub use crate::entity::scores::field::button::{
        ManiaButtonSet, StandardButton, StandardButtonSet,
    };
    pub use crate::entity::scores::field::replay::Replay;
}
