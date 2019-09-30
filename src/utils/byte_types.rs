use modular_bitfield::prelude::*;

#[bitfield]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BaseByte {
    a: bool, // Uses 1 bit
    b: bool, // Uses 9 bits
    c: B3,
    d: B3,
}

pub trait Convert {
    fn to_byte(&self) -> u8;
}

impl Convert for BaseByte {
    fn to_byte(&self) -> u8 {
        let mut x = 0u8;
        match self.get_a() {
            true => x |= 1 << 7,
            false => x |= 0 << 7,
        };
        match self.get_b() {
            true => x | (1 << 6),
            false => x | (0 << 6),
        };
        x |= (self.get_c() & 0b111) << 3;
        x |= self.get_d() & 0b111;
        return x;
    }
}
