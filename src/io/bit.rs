pub trait Bit {
    fn bit(&self, pos: u32) -> bool;
    fn bit_range(&self, pos: std::ops::Range<u32>) -> Self;
    fn set_bit(&mut self, pos: u32, val: bool);
    fn set_bit_range(&mut self, pos: std::ops::Range<u32>, val: Self);
}

macro_rules! impl_bit {
    (@ $ty:ty) => {
        impl Bit for $ty {
            fn bit(&self, pos: u32) -> bool {
                (*self & 1 << pos) != 0
            }
            fn bit_range(&self, pos: std::ops::Range<u32>) -> Self {
                (*self & ((1<<pos.end)-1)) >> pos.start
            }
            fn set_bit(&mut self, pos: u32, val: bool) {
                *self = (*self & !(1<<pos)) | ((val as Self)<<pos);
            }
            fn set_bit_range(&mut self, pos: std::ops::Range<u32>, val: Self) {
                let mask = ((1<<(pos.end-pos.start))-1) << pos.start;
                *self = (*self & !mask) | ((val<<pos.start)&mask);
            }
        }
    };
    ($($ty:ty),*) => {
        $(
            impl_bit!(@ $ty);
        )*
    }
}

impl_bit!(u8, u16, u32, u64);
