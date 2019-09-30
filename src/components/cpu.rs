#![allow(non_snake_case)]

use super::bus::*;
/// Struct representing the 6502 cpu's data
pub struct OLC6502 {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub stkp: u8,
    pub pc: u16,
    pub status: u8,
    pub data: u8,      // Data that can be fetched for some operations when needed
    pub addr_abs: u16, // Absolute Adress to another data source needed
    pub addr_rel: u16,
    pub curr_opcode: u8, // Opcode currently running
    pub cycles: u8,      // number of cycles left for the current opcode to finish
}
/// enum representing the various instruction flags
pub enum FLAGS6502 {
    C = 1 << 0, // Carry bit
    Z = 1 << 1, // Zero
    I = 1 << 2, // Disable interrupts
    D = 1 << 3, // Decimal mode
    B = 1 << 4, // Break
    U = 1 << 5, // Unused
    V = 1 << 6, // Overfow
    N = 1 << 7, // Negative
}
/// A struct representing instructions by name and corresponding function.
pub struct INSTRUCTION {
    pub name: String,
    pub cycles: u8,
}

/// Trait defining all the 6502 functions
pub trait CPUFunctions {
    fn get_flag(self, f: FLAGS6502) -> u8;
    fn set_flag(self, f: FLAGS6502, v: bool) -> ();

    // Addressing modes : specifies the way to get some data.
    /// Implied : the address containing the operands are implicity known
    fn IMP() -> u8;
    /// Immediate : addressing mode, the second byte of the instruction contains the operands
    fn IMM() -> u8;
    /// Zero Page : fetching only the second byte knowing the first one is zero. It looks for the 1st element in the instruction matrix. Performance
    fn ZP0() -> u8;
    /// Zero Page X : Adds only the second byte to the index range, faster adress accessing like ZP0    
    fn ZPX() -> u8;
    /// Zero Page Y : Adds only the second byte to the index range, faster adress accessing like ZP0    
    fn ZPY() -> u8;
    /// Relative : Used only for branch instructions and establish destination for the conditinal branch  
    fn REL() -> u8;
    /// Absolute : Second byte specifies the eight low order bits of the effective address while the third byte gives the high order bits. Thus making it possible to adress a wallopin 64K bytes of data
    fn ABS() -> u8;
    /// Absolute X : Used with the X register
    fn ABX() -> u8;
    /// Absolute Y : Used with the Y register
    fn ABY() -> u8;
    /// Absolute Indirect : Second byte gives the low order byte of the memory location, high order in third byte.
    fn IND() -> u8;
    /// Indirect indexed X : Indirect mode with use of the X register
    fn IZX() -> u8;
    /// Indirect indexed Y : Indirect mode with use of the Y register
    fn IZY() -> u8;

    
    fn ADC() -> u8;
    fn AND() -> u8;
    fn ASL() -> u8;
    fn BCC() -> u8;
    fn BCS() -> u8;
    fn BEQ() -> u8;
    fn BIT() -> u8;
    fn BMI() -> u8;
    fn BNE() -> u8;
    fn BPL() -> u8;
    fn BRK() -> u8;
    fn BVC() -> u8;
    fn BVS() -> u8;
    fn CLC() -> u8;
    fn CLD() -> u8;
    fn CLI() -> u8;
    fn CLV() -> u8;
    fn CMP() -> u8;
    fn CPX() -> u8;
    fn CPY() -> u8;
    fn DEC() -> u8;
    fn DEX() -> u8;
    fn DEY() -> u8;
    fn EOR() -> u8;
    fn INC() -> u8;
    fn INX() -> u8;
    fn INY() -> u8;
    fn JMP() -> u8;
    fn JSR() -> u8;
    fn LDA() -> u8;
    fn LDX() -> u8;
    fn NOP() -> u8;
    fn ORA() -> u8;
    fn PHA() -> u8;
    fn PHP() -> u8;
    fn PLA() -> u8;
    fn PLP() -> u8;
    fn ROL() -> u8;
    fn ROR() -> u8;
    fn RTI() -> u8;
    fn RTS() -> u8;
    fn SBC() -> u8;
    fn SEC() -> u8;
    fn SED() -> u8;
    fn SEI() -> u8;
    fn STA() -> u8;
    fn STX() -> u8;
    fn STY() -> u8;
    fn TAX() -> u8;
    fn TAY() -> u8;
    fn TSX() -> u8;
    fn TXA() -> u8;
    fn TXS() -> u8;
    fn TYA() -> u8;

    fn OpCodes(); // Unintended operations
                  // Those are the clock based functions
    fn clock(); // This should control the number of clock cycles each instructions takes.
    fn reset();
    fn interupt_req();
    fn non_maskable_interupt_req();
    fn fetch_data(); // it should fetch data
}

pub trait CpuIO {
    fn read(bus: &mut Bus, addr: usize, read_only: bool) -> u8;
    fn write(bus: &mut Bus, addr: usize, data: u8);
}

pub trait InstructionFunctions {
    fn operate(&self);
    fn addrmode(&self);
}

//TODO: Implement InstructionFunctions
// This trait should pattern match the name and call a specific instruction
// Should work as a pointer of function

impl OLC6502 {
    pub fn new() -> OLC6502 {
        OLC6502 {
            a: 0,
            x: 0,
            y: 0,
            stkp: 0,
            pc: 0,
            status: 0,
            data: 0,     // Data that can be fetched for some operations when needed
            addr_abs: 0, // Absolute Adress to another data source needed
            addr_rel: 0,
            curr_opcode: 0, // Opcode currently running
            cycles: 0,
        }
    }
}

impl CpuIO for OLC6502 {
    fn read(bus: &mut Bus, addr: usize, read_only: bool) -> u8 {
        //TODO: check if the address size is in the correct
        bus.read(addr, read_only)
    }
    fn write(bus: &mut Bus, addr: usize, data: u8) {
        bus.write(addr, data);
    }
}
