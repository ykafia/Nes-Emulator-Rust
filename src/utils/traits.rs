pub trait BitGet{
    fn get_high_bit(&self) -> bool;
    fn get_next_bit(&self) -> bool;
    fn get_low_bit(&self) -> bool;
    fn get_nth_bit(&self, n : u8) -> bool;
}
pub trait ByteGet{
    fn get_high_byte(&self) -> u8;
    fn get_low_byte(&self) -> u8;
}
impl BitGet for u8{
    fn get_high_bit(&self) -> bool{
        (self & 0x80) >> 7 == 1
    }
    fn get_next_bit(&self) -> bool{
        (self & 0x40) >> 6 == 1
    }
    fn get_low_bit(&self) -> bool{
        (self & 0x01) == 1
    }
    fn get_nth_bit(&self, n : u8) -> bool {
        ((self >> 7-n) & 0x01) == 1 
    }
}
impl BitGet for u16{
    fn get_high_bit(&self) -> bool{
        (self & 0x8000) >> 15 == 1
    }
    fn get_next_bit(&self) -> bool{
        (self & 0x4000) >> 14 == 1
    }
    fn get_low_bit(&self) -> bool{
        (self & 0x0001) == 1
    }
    fn get_nth_bit(&self, n : u8) -> bool {
        ((self >> 15-n) & 0x01) == 1 
    }
}
impl ByteGet for u16{
    fn get_low_byte(&self) -> u8{
        (self & 0x00FF) as u8
    }
    fn get_high_byte(&self) -> u8{
        ((self & 0xFF00) >> 8) as u8
    }

}
pub trait OverflowOp<T>{
    fn add_overflow(&self,value : T) -> T;
    fn sub_overflow(&self,value : T) -> T;
}
impl OverflowOp<u8> for u8{
    fn add_overflow(&self, value : u8) -> u8{
        match self.checked_add(value){
            Some(x) => x,
            None => value - (std::u8::MAX - self)
        }
    }
    fn sub_overflow(&self, value : u8) -> u8{
        match self.checked_sub(value){
            Some(x) => x,
            None => std::u8::MAX - (value - self)
        }
    }
}