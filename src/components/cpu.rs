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

pub trait OpCodesCaller {
    fn apply(&self, cpu: &mut OLC6502) -> u8;
}

/// Trait defining all the 6502 functions
pub trait CPUFunctions {
    fn get_flag(self, f: FLAGS6502) -> u8;
    fn set_flag(self, f: FLAGS6502, v: bool) -> ();

    /// Clock management function
    /// This should control the number of clock cycles each instructions takes.
    fn clock();
    fn reset();
    fn interupt_req();
    fn non_maskable_interupt_req();
    fn fetch_data();
}
pub trait CpuIO {
    fn read(&self, bus: &mut Bus, addr: usize, read_only: bool) -> u8;
    fn write(&self, bus: &mut Bus, addr: usize, data: u8);
}

pub trait InstructionFunctions {
    fn operate(&self);
    fn addrmode(&self);
}

pub trait AdressingModes {
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
}

pub trait OperationCodes {
    fn ADC(&self) -> u8;
    fn AND(&self) -> u8;
    fn ASL(&self) -> u8;
    fn BCC(&self) -> u8;
    fn BCS(&self) -> u8;
    fn BEQ(&self) -> u8;
    fn BIT(&self) -> u8;
    fn BMI(&self) -> u8;
    fn BNE(&self) -> u8;
    fn BPL(&self) -> u8;
    fn BRK(&self) -> u8;
    fn BVC(&self) -> u8;
    fn BVS(&self) -> u8;
    fn CLC(&self) -> u8;
    fn CLD(&self) -> u8;
    fn CLI(&self) -> u8;
    fn CLV(&self) -> u8;
    fn CMP(&self) -> u8;
    fn CPX(&self) -> u8;
    fn CPY(&self) -> u8;
    fn DEC(&self) -> u8;
    fn DEX(&self) -> u8;
    fn DEY(&self) -> u8;
    fn EOR(&self) -> u8;
    fn INC(&self) -> u8;
    fn INX(&self) -> u8;
    fn INY(&self) -> u8;
    fn JMP(&self) -> u8;
    fn JSR(&self) -> u8;
    fn LDA(&self) -> u8;
    fn LDX(&self) -> u8;
    fn NOP(&self) -> u8;
    fn ORA(&self) -> u8;
    fn PHA(&self) -> u8;
    fn PHP(&self) -> u8;
    fn PLA(&self) -> u8;
    fn PLP(&self) -> u8;
    fn ROL(&self) -> u8;
    fn ROR(&self) -> u8;
    fn RTI(&self) -> u8;
    fn RTS(&self) -> u8;
    fn SBC(&self) -> u8;
    fn SEC(&self) -> u8;
    fn SED(&self) -> u8;
    fn SEI(&self) -> u8;
    fn STA(&self) -> u8;
    fn STX(&self) -> u8;
    fn STY(&self) -> u8;
    fn TAX(&self) -> u8;
    fn TAY(&self) -> u8;
    fn TSX(&self) -> u8;
    fn TXA(&self) -> u8;
    fn TXS(&self) -> u8;
    fn TYA(&self) -> u8;

    fn XXX(&self) -> u8; // Unintended operations
}





//#######################################################################################
//#                         IMPLEMENTATION OF TRAITS                                    #
//#             Gonna be a long, long way till the EOF, just bear with it               #
//#######################################################################################



impl OLC6502 {
    pub fn new() -> OLC6502 {
        OLC6502 {
            a: 0,    // a register
            x: 0,    // x register for low index
            y: 0,    // y register for high index
            stkp: 0, // stack pointer
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
    fn read(&self, bus: &mut Bus, addr: usize, read_only: bool) -> u8 {
        //TODO: check if the address size is in the correct
        bus.read(addr, read_only)
    }
    fn write(&self, bus: &mut Bus, addr: usize, data: u8) {
        bus.write(addr, data);
    }
}

impl AdressingModes for OLC6502{
    fn IMP() -> u8{
        0u8
    }
    fn IMM() -> u8{
        0u8
    }
    fn ZP0() -> u8{
        0u8
    }
    fn ZPX() -> u8{
        0u8
    }
    fn ZPY() -> u8{
        0u8
    }
    fn REL() -> u8{
        0u8
    }
    fn ABS() -> u8{
        0u8
    }
    fn ABX() -> u8{
        0u8
    }
    fn ABY() -> u8{
        0u8
    }
    fn IND() -> u8{
        0u8
    }
    fn IZX() -> u8{
        0u8
    }
    fn IZY() -> u8{
        0u8
    }
}

impl OpCodesCaller for INSTRUCTION {
    fn apply(&self, cpu: &mut OLC6502) -> u8 {
        match self.name.as_str() {
            "ADC" => cpu.ADC(),
            "AND" => cpu.AND(),
            "ASL" => cpu.ASL(),
            "BCC" => cpu.BCC(),
            "BCS" => cpu.BCS(),
            "BEQ" => cpu.BEQ(),
            "BIT" => cpu.BIT(),
            "BMI" => cpu.BMI(),
            "BNE" => cpu.BNE(),
            "BPL" => cpu.BPL(),
            "BRK" => cpu.BRK(),
            "BVC" => cpu.BVC(),
            "BVS" => cpu.BVS(),
            "CLC" => cpu.CLC(),
            "CLD" => cpu.CLD(),
            "CLI" => cpu.CLI(),
            "CLV" => cpu.CLV(),
            "CMP" => cpu.CMP(),
            "CPX" => cpu.CPX(),
            "CPY" => cpu.CPY(),
            "DEC" => cpu.DEC(),
            "DEX" => cpu.DEX(),
            "DEY" => cpu.DEY(),
            "EOR" => cpu.EOR(),
            "INC" => cpu.INC(),
            "INX" => cpu.INX(),
            "INY" => cpu.INY(),
            "JMP" => cpu.JMP(),
            "JSR" => cpu.JSR(),
            "LDA" => cpu.LDA(),
            "LDX" => cpu.LDX(),
            "NOP" => cpu.NOP(),
            "ORA" => cpu.ORA(),
            "PHA" => cpu.PHA(),
            "PHP" => cpu.PHP(),
            "PLA" => cpu.PLA(),
            "PLP" => cpu.PLP(),
            "ROL" => cpu.ROL(),
            "ROR" => cpu.ROR(),
            "RTI" => cpu.RTI(),
            "RTS" => cpu.RTS(),
            "SBC" => cpu.SBC(),
            "SEC" => cpu.SEC(),
            "SED" => cpu.SED(),
            "SEI" => cpu.SEI(),
            "STA" => cpu.STA(),
            "STX" => cpu.STX(),
            "STY" => cpu.STY(),
            "TAX" => cpu.TAX(),
            "TAY" => cpu.TAY(),
            "TSX" => cpu.TSX(),
            "TXA" => cpu.TXA(),
            "TXS" => cpu.TXS(),
            "TYA" => cpu.TYA(),
            _ => cpu.XXX(), // Unintended operations
        }
    }
}

impl OperationCodes for OLC6502 {
    fn ADC(&self) -> u8 {
        0u8
    }
    fn AND(&self) -> u8 {
        0u8
    }
    fn ASL(&self) -> u8 {
        0u8
    }
    fn BCC(&self) -> u8 {
        0u8
    }
    fn BCS(&self) -> u8 {
        0u8
    }
    fn BEQ(&self) -> u8 {
        0u8
    }
    fn BIT(&self) -> u8 {
        0u8
    }
    fn BMI(&self) -> u8 {
        0u8
    }
    fn BNE(&self) -> u8 {
        0u8
    }
    fn BPL(&self) -> u8 {
        0u8
    }
    fn BRK(&self) -> u8 {
        0u8
    }
    fn BVC(&self) -> u8 {
        0u8
    }
    fn BVS(&self) -> u8 {
        0u8
    }
    fn CLC(&self) -> u8 {
        0u8
    }
    fn CLD(&self) -> u8 {
        0u8
    }
    fn CLI(&self) -> u8 {
        0u8
    }
    fn CLV(&self) -> u8 {
        0u8
    }
    fn CMP(&self) -> u8 {
        0u8
    }
    fn CPX(&self) -> u8 {
        0u8
    }
    fn CPY(&self) -> u8 {
        0u8
    }
    fn DEC(&self) -> u8 {
        0u8
    }
    fn DEX(&self) -> u8 {
        0u8
    }
    fn DEY(&self) -> u8 {
        0u8
    }
    fn EOR(&self) -> u8 {
        0u8
    }
    fn INC(&self) -> u8 {
        0u8
    }
    fn INX(&self) -> u8 {
        0u8
    }
    fn INY(&self) -> u8 {
        0u8
    }
    fn JMP(&self) -> u8 {
        0u8
    }
    fn JSR(&self) -> u8 {
        0u8
    }
    fn LDA(&self) -> u8 {
        0u8
    }
    fn LDX(&self) -> u8 {
        0u8
    }
    fn NOP(&self) -> u8 {
        0u8
    }
    fn ORA(&self) -> u8 {
        0u8
    }
    fn PHA(&self) -> u8 {
        0u8
    }
    fn PHP(&self) -> u8 {
        0u8
    }
    fn PLA(&self) -> u8 {
        0u8
    }
    fn PLP(&self) -> u8 {
        0u8
    }
    fn ROL(&self) -> u8 {
        0u8
    }
    fn ROR(&self) -> u8 {
        0u8
    }
    fn RTI(&self) -> u8 {
        0u8
    }
    fn RTS(&self) -> u8 {
        0u8
    }
    fn SBC(&self) -> u8 {
        0u8
    }
    fn SEC(&self) -> u8 {
        0u8
    }
    fn SED(&self) -> u8 {
        0u8
    }
    fn SEI(&self) -> u8 {
        0u8
    }
    fn STA(&self) -> u8 {
        0u8
    }
    fn STX(&self) -> u8 {
        0u8
    }
    fn STY(&self) -> u8 {
        0u8
    }
    fn TAX(&self) -> u8 {
        0u8
    }
    fn TAY(&self) -> u8 {
        0u8
    }
    fn TSX(&self) -> u8 {
        0u8
    }
    fn TXA(&self) -> u8 {
        0u8
    }
    fn TXS(&self) -> u8 {
        0u8
    }
    fn TYA(&self) -> u8 {
        0u8
    }

    fn XXX(&self) -> u8 {
        0u8
    }
}
