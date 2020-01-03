use super::*;
pub trait BitGet {
    fn get_high_bit(&self) -> bool;
    fn get_next_bit(&self) -> bool;
    fn get_low_bit(&self) -> bool;
    fn get_nth_bit(&self, n: u8) -> bool;
}
pub trait ByteGet {
    fn get_high_byte(&self) -> u8;
    fn get_low_byte(&self) -> u8;
}
impl BitGet for u8 {
    fn get_high_bit(&self) -> bool {
        (self & 0x80) >> 7 == 1
    }
    fn get_next_bit(&self) -> bool {
        (self & 0x40) >> 6 == 1
    }
    fn get_low_bit(&self) -> bool {
        (self & 0x01) == 1
    }
    fn get_nth_bit(&self, n: u8) -> bool {
        ((self >> 7 - n) & 0x01) == 1
    }
}
impl BitGet for u16 {
    fn get_high_bit(&self) -> bool {
        (self & 0x8000) >> 15 == 1
    }
    fn get_next_bit(&self) -> bool {
        (self & 0x4000) >> 14 == 1
    }
    fn get_low_bit(&self) -> bool {
        (self & 0x0001) == 1
    }
    fn get_nth_bit(&self, n: u8) -> bool {
        ((self >> 15 - n) & 0x01) == 1
    }
}
impl ByteGet for u16 {
    fn get_low_byte(&self) -> u8 {
        (self & 0x00FF) as u8
    }
    fn get_high_byte(&self) -> u8 {
        ((self & 0xFF00) >> 8) as u8
    }
}
pub trait OverflowOp<T> {
    fn add_overflow(&self, value: T) -> T;
    fn sub_overflow(&self, value: T) -> T;
}
impl OverflowOp<u8> for u8 {
    fn add_overflow(&self, value: u8) -> u8 {
        match self.checked_add(value) {
            Some(x) => x,
            None => value - (std::u8::MAX - self),
        }
    }
    fn sub_overflow(&self, value: u8) -> u8 {
        match self.checked_sub(value) {
            Some(x) => x,
            None => std::u8::MAX - (value - self),
        }
    }
}

pub trait InBetween<T> {
    /// Provide an inbetween check.
    /// Low number is included, high number is excluded
    fn in_between(&self, low: T, high: T) -> bool;
}

/// Traits that gives the components to where you should read or write data.
/// Typically if you're in the NESData you want to know if you're reading from the cartridge,
/// the ram or the mapper. T here is supposed to be en ENUM that you have to match to read and write
pub trait AddrWhere<T> {
    fn to_where(&self) -> T;
}

pub trait ReadWriteFunc{
    fn cpu_read(nes: &mut NesData, addr: u16, read_only: bool) -> u8;
    fn cpu_write( nes: &mut NesData, addr: u16, data: u8) ;
    fn ppu_read(ppu: &mut PPU, addr: u16, read_only: bool) -> u8;
    fn ppu_write( ppu: &mut PPU, addr: u16, data: u8) ;
}

