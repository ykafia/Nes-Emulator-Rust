#![allow(non_snake_case)]

use super::bus::*;
use super::instruction_generator::get_lookup_list;
use std::convert::TryInto;

//use ndarray::Array2;

/// Struct representing the 6502 cpu's data
pub struct OLC6502 {
    /// Accumulator :
    /// A is byte-wide and along with the arithmetic logic unit (ALU),
    /// supports using the status register for carrying,
    /// overflow detection, and so on.
    pub a: u8,
    /// Indexes X & Y
    pub x: u8,
    pub y: u8,
    /// Stack Pointer
    /// S is byte-wide and can be accessed using interrupts,
    /// pulls, pushes, and transfers.
    pub stkp: u8,
    /// Program Counter :
    /// The 2-byte program counter PC supports 65536 direct (unbanked) memory
    /// locations, however not all values are sent to the cartridge.
    /// It can be accessed either by allowing CPU's internal fetch logic
    /// increment the address bus, an interrupt (NMI, Reset, IRQ/BRQ),
    /// and using the RTS/JMP/JSR/Branch instructions.
    pub pc: u16,
    /// Status Register :
    /// P has 6 bits used by the ALU but is byte-wide.
    /// PHP, PLP, arithmetic, testing, and branch instructions can access this register.
    pub status: u8,
    /// Data that can be fetched for some operations when needed
    pub fetched_data: u8,
    /// Absolute Adress to another data source needed
    pub addr_abs: u16,
    /// Relative Adress to another data source needed
    pub addr_rel: u16,
    /// Opcode currently running
    pub curr_opcode: u8,
    /// number of cycles left for the current opcode to finish
    pub cycles: u8,

    pub lookup: Vec<Vec<INSTRUCTION>>,
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

#[derive(Clone)]
pub struct INSTRUCTION {
    pub opcode: String,
    pub addr_mode: String,
    pub cycles: u8,
}

// pub trait InstructionFunctions {
//     fn apply_op(&mut self, cpu: &OLC6502) -> u8;
//     fn apply_addressing_mode(&mut self, cpu:  &OLC6502) -> u8;
// }

pub trait CpuApplyFunctions {
    fn apply_op(&mut self, instruction: INSTRUCTION, bus: &mut Bus) -> u8;
    fn apply_addressing_mode(&mut self, instruction: INSTRUCTION, bus: &mut Bus) -> u8;
}

/// Trait defining all the 6502 functions
pub trait CPUFunctions {
    fn get_flag(&mut self, f: FLAGS6502) -> u8;
    fn set_flag(&mut self, f: FLAGS6502, v: bool) -> ();

    /// Clock management function
    /// This should control the number of clock cycles each instructions takes.
    fn clock(&mut self, but: &mut Bus);
    fn reset(&mut self);
    fn interupt_req(&mut self);
    fn non_maskable_interupt_req(&mut self);
    fn fetch_data(&mut self);
}
pub trait CpuIO {
    fn read(&mut self, bus: &mut Bus, addr: u16, read_only: bool) -> u8;
    fn write(&mut self, bus: &mut Bus, addr: u16, data: u8);
}

pub trait AddressingModes {
    // Addressing modes : specifies the way to get some data.
    /// Implied : the address containing the operands are implicity known
    fn IMP(&mut self) -> u8;
    /// Immediate : addressing mode, the second byte of the instruction contains the operands
    fn IMM(&mut self) -> u8;
    /// Zero Page : fetching only the second byte knowing the first one is zero. It looks for the 1st element in the instruction matrix. Performance
    fn ZP0(&mut self) -> u8;
    /// Zero Page X : Adds only the second byte to the index range, faster adress accessing like ZP0    
    fn ZPX(&mut self, bus: &mut Bus) -> u8;
    /// Zero Page Y : Adds only the second byte to the index range, faster adress accessing like ZP0    
    fn ZPY(&mut self, bus: &mut Bus) -> u8;
    /// Relative : Used only for branch instructions and establish destination for the conditinal branch  
    fn REL(&mut self) -> u8;
    /// Absolute : Second byte specifies the eight low order bits of the effective address while the third byte gives the high order bits. Thus making it possible to adress a wallopin 64K bytes of data
    fn ABS(&mut self) -> u8;
    /// Absolute X : Used with the X register
    fn ABX(&mut self) -> u8;
    /// Absolute Y : Used with the Y register
    fn ABY(&mut self) -> u8;
    /// Absolute Indirect : Second byte gives the low order byte of the memory location, high order in third byte.
    fn IND(&mut self) -> u8;
    /// Indirect indexed X : Indirect mode with use of the X register
    fn IZX(&mut self) -> u8;
    /// Indirect indexed Y : Indirect mode with use of the Y register
    fn IZY(&mut self) -> u8;
}

pub trait OperationCodes {
    fn ADC(&mut self) -> u8;
    fn AND(&mut self) -> u8;
    fn ASL(&mut self) -> u8;
    fn BCC(&mut self) -> u8;
    fn BCS(&mut self) -> u8;
    fn BEQ(&mut self) -> u8;
    fn BIT(&mut self) -> u8;
    fn BMI(&mut self) -> u8;
    fn BNE(&mut self) -> u8;
    fn BPL(&mut self) -> u8;
    fn BRK(&mut self) -> u8;
    fn BVC(&mut self) -> u8;
    fn BVS(&mut self) -> u8;
    fn CLC(&mut self) -> u8;
    fn CLD(&mut self) -> u8;
    fn CLI(&mut self) -> u8;
    fn CLV(&mut self) -> u8;
    fn CMP(&mut self) -> u8;
    fn CPX(&mut self) -> u8;
    fn CPY(&mut self) -> u8;
    fn DEC(&mut self) -> u8;
    fn DEX(&mut self) -> u8;
    fn DEY(&mut self) -> u8;
    fn EOR(&mut self) -> u8;
    fn INC(&mut self) -> u8;
    fn INX(&mut self) -> u8;
    fn INY(&mut self) -> u8;
    fn JMP(&mut self) -> u8;
    fn JSR(&mut self) -> u8;
    fn LDA(&mut self) -> u8;
    fn LDX(&mut self) -> u8;
    fn NOP(&mut self) -> u8;
    fn ORA(&mut self) -> u8;
    fn PHA(&mut self) -> u8;
    fn PHP(&mut self) -> u8;
    fn PLA(&mut self) -> u8;
    fn PLP(&mut self) -> u8;
    fn ROL(&mut self) -> u8;
    fn ROR(&mut self) -> u8;
    fn RTI(&mut self) -> u8;
    fn RTS(&mut self) -> u8;
    fn SBC(&mut self) -> u8;
    fn SEC(&mut self) -> u8;
    fn SED(&mut self) -> u8;
    fn SEI(&mut self) -> u8;
    fn STA(&mut self) -> u8;
    fn STX(&mut self) -> u8;
    fn STY(&mut self) -> u8;
    fn TAX(&mut self) -> u8;
    fn TAY(&mut self) -> u8;
    fn TSX(&mut self) -> u8;
    fn TXA(&mut self) -> u8;
    fn TXS(&mut self) -> u8;
    fn TYA(&mut self) -> u8;

    fn XXX(&mut self) -> u8; // Unintended operations
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
            fetched_data: 0, // Data that can be fetched for some operations when needed
            addr_abs: 0,     // Absolute Adress to another data source needed
            addr_rel: 0,
            curr_opcode: 0, // Opcode currently running
            cycles: 0,
            lookup: get_lookup_list(),
        }
    }
}

impl CpuIO for OLC6502 {
    fn read(&mut self, bus: &mut Bus, addr: u16, read_only: bool) -> u8 {
        //TODO: check if the address size is in the correct
        bus.read(addr, read_only)
    }
    fn write(&mut self, bus: &mut Bus, addr: u16, data: u8) {
        bus.write(addr, data);
    }
}

impl AddressingModes for OLC6502 {
    fn IMP(&mut self) -> u8 {
        self.fetched_data = self.a;
        0u8
    }
    fn IMM(&mut self) -> u8 {
        self.addr_abs = self.pc + 1;
        0u8
    }
    fn ZP0(&mut self) -> u8 {
        self.addr_abs = self.pc;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        0u8
    }
    fn ZPX(&mut self, bus: &mut Bus) -> u8 {
        self.addr_abs = (self.read(bus, self.pc, true) + self.x).into();
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        0u8
    }
    fn ZPY(&mut self, bus: &mut Bus) -> u8 {
        self.addr_abs = (self.read(bus, self.pc, true) + self.y).into();
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        0u8
    }
    fn REL(&mut self) -> u8 {
        0u8
    }
    fn ABS(&mut self) -> u8 {
        0u8
    }
    fn ABX(&mut self) -> u8 {
        0u8
    }
    fn ABY(&mut self) -> u8 {
        0u8
    }
    fn IND(&mut self) -> u8 {
        0u8
    }
    fn IZX(&mut self) -> u8 {
        0u8
    }
    fn IZY(&mut self) -> u8 {
        0u8
    }
}

impl CpuApplyFunctions for OLC6502 {
    fn apply_op(&mut self, instruction: INSTRUCTION, bus: &mut Bus) -> u8 {
        match instruction.opcode.as_str() {
            "ADC" => self.ADC(),
            "AND" => self.AND(),
            "ASL" => self.ASL(),
            "BCC" => self.BCC(),
            "BCS" => self.BCS(),
            "BEQ" => self.BEQ(),
            "BIT" => self.BIT(),
            "BMI" => self.BMI(),
            "BNE" => self.BNE(),
            "BPL" => self.BPL(),
            "BRK" => self.BRK(),
            "BVC" => self.BVC(),
            "BVS" => self.BVS(),
            "CLC" => self.CLC(),
            "CLD" => self.CLD(),
            "CLI" => self.CLI(),
            "CLV" => self.CLV(),
            "CMP" => self.CMP(),
            "CPX" => self.CPX(),
            "CPY" => self.CPY(),
            "DEC" => self.DEC(),
            "DEX" => self.DEX(),
            "DEY" => self.DEY(),
            "EOR" => self.EOR(),
            "INC" => self.INC(),
            "INX" => self.INX(),
            "INY" => self.INY(),
            "JMP" => self.JMP(),
            "JSR" => self.JSR(),
            "LDA" => self.LDA(),
            "LDX" => self.LDX(),
            "NOP" => self.NOP(),
            "ORA" => self.ORA(),
            "PHA" => self.PHA(),
            "PHP" => self.PHP(),
            "PLA" => self.PLA(),
            "PLP" => self.PLP(),
            "ROL" => self.ROL(),
            "ROR" => self.ROR(),
            "RTI" => self.RTI(),
            "RTS" => self.RTS(),
            "SBC" => self.SBC(),
            "SEC" => self.SEC(),
            "SED" => self.SED(),
            "SEI" => self.SEI(),
            "STA" => self.STA(),
            "STX" => self.STX(),
            "STY" => self.STY(),
            "TAX" => self.TAX(),
            "TAY" => self.TAY(),
            "TSX" => self.TSX(),
            "TXA" => self.TXA(),
            "TXS" => self.TXS(),
            "TYA" => self.TYA(),
            _ => self.XXX(), // Unintended operations
        }
    }
    fn apply_addressing_mode(&mut self, instruction: INSTRUCTION, bus: &mut Bus) -> u8 {
        match instruction.addr_mode.as_str() {
            "IMP" => self.IMP(),
            "IMM" => self.IMM(),
            "ZP0" => self.ZP0(),
            "ZPX" => self.ZPX(bus),
            "ZPY" => self.ZPY(bus),
            "REL" => self.REL(),
            "ABS" => self.ABS(),
            "ABX" => self.ABX(),
            "ABY" => self.ABY(),
            "IND" => self.IND(),
            "IZX" => self.IZX(),
            "IZY" => self.IZY(),
            _ => 0u8,
        }
    }
}

impl OperationCodes for OLC6502 {
    fn ADC(&mut self) -> u8 {
        0u8
    }
    fn AND(&mut self) -> u8 {
        0u8
    }
    fn ASL(&mut self) -> u8 {
        0u8
    }
    fn BCC(&mut self) -> u8 {
        0u8
    }
    fn BCS(&mut self) -> u8 {
        0u8
    }
    fn BEQ(&mut self) -> u8 {
        0u8
    }
    fn BIT(&mut self) -> u8 {
        0u8
    }
    fn BMI(&mut self) -> u8 {
        0u8
    }
    fn BNE(&mut self) -> u8 {
        0u8
    }
    fn BPL(&mut self) -> u8 {
        0u8
    }
    fn BRK(&mut self) -> u8 {
        0u8
    }
    fn BVC(&mut self) -> u8 {
        0u8
    }
    fn BVS(&mut self) -> u8 {
        0u8
    }
    fn CLC(&mut self) -> u8 {
        0u8
    }
    fn CLD(&mut self) -> u8 {
        0u8
    }
    fn CLI(&mut self) -> u8 {
        0u8
    }
    fn CLV(&mut self) -> u8 {
        0u8
    }
    fn CMP(&mut self) -> u8 {
        0u8
    }
    fn CPX(&mut self) -> u8 {
        0u8
    }
    fn CPY(&mut self) -> u8 {
        0u8
    }
    fn DEC(&mut self) -> u8 {
        0u8
    }
    fn DEX(&mut self) -> u8 {
        0u8
    }
    fn DEY(&mut self) -> u8 {
        0u8
    }
    fn EOR(&mut self) -> u8 {
        0u8
    }
    fn INC(&mut self) -> u8 {
        0u8
    }
    fn INX(&mut self) -> u8 {
        0u8
    }
    fn INY(&mut self) -> u8 {
        0u8
    }
    fn JMP(&mut self) -> u8 {
        0u8
    }
    fn JSR(&mut self) -> u8 {
        0u8
    }
    fn LDA(&mut self) -> u8 {
        0u8
    }
    fn LDX(&mut self) -> u8 {
        0u8
    }
    fn NOP(&mut self) -> u8 {
        0u8
    }
    fn ORA(&mut self) -> u8 {
        0u8
    }
    fn PHA(&mut self) -> u8 {
        0u8
    }
    fn PHP(&mut self) -> u8 {
        0u8
    }
    fn PLA(&mut self) -> u8 {
        0u8
    }
    fn PLP(&mut self) -> u8 {
        0u8
    }
    fn ROL(&mut self) -> u8 {
        0u8
    }
    fn ROR(&mut self) -> u8 {
        0u8
    }
    fn RTI(&mut self) -> u8 {
        0u8
    }
    fn RTS(&mut self) -> u8 {
        0u8
    }
    fn SBC(&mut self) -> u8 {
        0u8
    }
    fn SEC(&mut self) -> u8 {
        0u8
    }
    fn SED(&mut self) -> u8 {
        0u8
    }
    fn SEI(&mut self) -> u8 {
        0u8
    }
    fn STA(&mut self) -> u8 {
        0u8
    }
    fn STX(&mut self) -> u8 {
        0u8
    }
    fn STY(&mut self) -> u8 {
        0u8
    }
    fn TAX(&mut self) -> u8 {
        0u8
    }
    fn TAY(&mut self) -> u8 {
        0u8
    }
    fn TSX(&mut self) -> u8 {
        0u8
    }
    fn TXA(&mut self) -> u8 {
        0u8
    }
    fn TXS(&mut self) -> u8 {
        0u8
    }
    fn TYA(&mut self) -> u8 {
        0u8
    }

    fn XXX(&mut self) -> u8 {
        0u8
    }
}

impl CPUFunctions for OLC6502 {
    fn clock(&mut self, bus: &mut Bus) {
        if self.cycles == 0 {
            self.curr_opcode = self.read(bus, self.pc.try_into().unwrap(), true);
            self.pc += 1;
            let high: usize = (self.curr_opcode | 0xF0 >> 4).try_into().unwrap();
            let low: usize = (self.curr_opcode | 0x0F).try_into().unwrap();
            let additionnal_cycle_1 = self.apply_op(self.lookup[low][high].clone(), bus);
            let additionnal_cycle_2 =
                self.apply_addressing_mode(self.lookup[low][high].clone(), bus);
            self.cycles += additionnal_cycle_1 & additionnal_cycle_2;
        }
        self.cycles -= 1;
    }
    fn get_flag(&mut self, f: FLAGS6502) -> u8 {
        0u8
    }
    fn set_flag(&mut self, f: FLAGS6502, v: bool) {}
    fn reset(&mut self) {}
    fn interupt_req(&mut self) {}
    fn fetch_data(&mut self) {}
    fn non_maskable_interupt_req(&mut self) {}
}
