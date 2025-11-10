use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
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
