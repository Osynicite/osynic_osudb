use crate::entity::osu::field::modification::ModSet;

#[cfg_attr(feature = "export", tsify::declare)]
pub type StarRatings = Vec<(ModSet, f64)>;
