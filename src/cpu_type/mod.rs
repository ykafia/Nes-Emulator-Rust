#![allow(non_snake_case)]
use super::common_traits::*;

trait CPUFunctions{
    
    fn get_flag(self,f:FLAGS6502)->u8;
    fn set_flag(self,f:FLAGS6502,v:bool)->();
    
    fn IMP() ->u8;    
    fn IMM()->u8;
    fn ZP0()->u8;    
    fn ZPX()->u8;
    fn ZPY()->u8;    
    fn REL()->u8;
    fn ABS()->u8;    
    fn ABX()->u8;
    fn ABY()->u8;
    fn IND()->u8;
    fn IZX()->u8;
    fn IZY()->u8;
}

struct CPU {
    a: u8,
    x: u8,
    y: u8,
    stkp: u8,
    pc: u16,
    status: u8,
}
enum FLAGS6502 {
    C = 1 << 0, // Carry bit
    Z = 1 << 1, // Zero
    I = 1 << 2, // Disable interrupts
    D = 1 << 3, // Decimal mode
    B = 1 << 4, // Break
    U = 1 << 5, // Unused
    V = 1 << 6, // Overfow
    N = 1 << 7, // Negative
}

