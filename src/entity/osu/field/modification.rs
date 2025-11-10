// Oh this is a true "Mod" module ~ but not rusty qwq
use crate::io::bit::Bit;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    Nightcore,
    Flashlight,
    Autoplay,
    SpunOut,
    Autopilot,
    Perfect,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    FadeIn,
    Random,
    LastMod,
    TargetPractice,
    Key9,
    Coop,
    Key1,
    Key3,
    Key2,
}
impl Mod {
    pub fn raw(&self) -> u8 {
        *self as u8
    }
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

#[cfg_attr(feature = "export", derive(tsify::Tsify))]
#[cfg_attr(feature = "export", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModSet(pub u32);
impl ModSet {
    pub fn bits(&self) -> u32 {
        self.0
    }
    pub fn from_bits(bits: u32) -> ModSet {
        ModSet(bits)
    }
    pub fn empty() -> ModSet {
        ModSet::from_bits(0)
    }
    pub fn contains(&self, m: Mod) -> bool {
        self.bits().bit(m.raw() as u32)
    }
    pub fn set(&self, m: Mod, include: bool) -> ModSet {
        let mut bits = self.bits();
        bits.set_bit(m.raw() as u32, include);
        ModSet::from_bits(bits)
    }
    pub fn with(&self, m: Mod) -> ModSet {
        self.set(m, true)
    }
    pub fn without(&self, m: Mod) -> ModSet {
        self.set(m, false)
    }
}
