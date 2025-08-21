use crate::io::bit::Bit;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u32)]
pub enum StandardButton {
    MousePrimary,
    MouseSecondary,
    KeyPrimary,
    KeySecondary,
}
impl StandardButton {
    pub fn raw(&self) -> u32 {
        *self as u32
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct StandardButtonSet(pub u32);
impl StandardButtonSet {
    pub fn bits(self) -> u32 {
        self.0
    }
    pub fn from_bits(bits: u32) -> StandardButtonSet {
        StandardButtonSet(bits)
    }
    pub fn none() -> StandardButtonSet {
        StandardButtonSet::from_bits(0)
    }
    pub fn is_down(&self, button: StandardButton) -> bool {
        self.bits().bit(button.raw())
    }
    pub fn set_down(&self, button: StandardButton, is_down: bool) -> StandardButtonSet {
        let mut bits = self.bits();
        bits.set_bit(button.raw(), is_down);
        StandardButtonSet::from_bits(bits)
    }
    pub fn press(&self, button: StandardButton) -> StandardButtonSet {
        self.set_down(button, true)
    }
    pub fn release(&self, button: StandardButton) -> StandardButtonSet {
        self.set_down(button, false)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ManiaButtonSet(pub u32);
impl ManiaButtonSet {
    pub fn bits(&self) -> u32 {
        self.0
    }
    pub fn from_bits(bits: u32) -> ManiaButtonSet {
        ManiaButtonSet(bits)
    }
    pub fn none() -> ManiaButtonSet {
        ManiaButtonSet::from_bits(0)
    }
    pub fn is_down(&self, button: u32) -> bool {
        self.bits().bit(button)
    }
    pub fn set_down(&self, button: u32, is_down: bool) -> ManiaButtonSet {
        let mut bits = self.bits();
        bits.set_bit(button, is_down);
        ManiaButtonSet::from_bits(bits)
    }
    pub fn press(&self, button: u32) -> ManiaButtonSet {
        self.set_down(button, true)
    }
    pub fn release(&self, button: u32) -> ManiaButtonSet {
        self.set_down(button, false)
    }
}
