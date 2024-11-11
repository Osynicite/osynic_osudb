use serde::{Deserialize, Serialize};

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