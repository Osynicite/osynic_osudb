use serde::{Deserialize, Serialize};

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
