#![allow(non_snake_case)]

use super::bus::*;
use super::instruction_generator::get_lookup_list;
use super::super::utils::*;
//TODO: Implement the rest of the cpu instructions

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

    pub lookup: Vec<INSTRUCTION>,
}
/// enum representing the various instruction flags
pub enum FLAGS6502 {
    /// Carry bit
    C = 1 << 0,
    /// Zero
    Z = 1 << 1,
    /// Disable interrupts
    I = 1 << 2,
    /// Decimal mode
    D = 1 << 3,
    /// Break
    B = 1 << 4,
    /// Unused
    U = 1 << 5,
    /// Overfow
    V = 1 << 6,
    /// Negative
    N = 1 << 7,
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
    fn clock(&mut self, bus: &mut Bus);
    fn reset(&mut self, bus: &mut Bus);
    fn power(&mut self, bus: &mut Bus);
    fn interupt_req(&mut self, bus: &mut Bus);
    fn non_maskable_interupt_req(&mut self, bus: &mut Bus);
    fn fetch_data(&mut self, bus: &mut Bus) -> u8;

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
    fn REL(&mut self, bus: &mut Bus) -> u8;
    /// Absolute : Second byte specifies the eight low order bits of the effective address while the third byte gives the high order bits. Thus making it possible to adress a wallopin 64K bytes of data
    fn ABS(&mut self, bus: &mut Bus) -> u8;
    /// Absolute X : Used with the X register
    fn ABX(&mut self, bus: &mut Bus) -> u8;
    /// Absolute Y : Used with the Y register
    fn ABY(&mut self, bus: &mut Bus) -> u8;
    /// Absolute Indirect : Second byte gives the low order byte of the memory location, high order in third byte.
    fn IND(&mut self, bus: &mut Bus) -> u8;
    /// Indirect indexed X : Indirect mode with use of the X register
    fn IZX(&mut self, bus: &mut Bus) -> u8;
    /// Indirect indexed Y : Indirect mode with use of the Y register
    fn IZY(&mut self, bus: &mut Bus) -> u8;
}

pub trait OperationCodes {
    fn ADC(&mut self, bus: &mut Bus) -> u8;
    fn AND(&mut self, bus: &mut Bus) -> u8;
    fn ASL(&mut self, bus: &mut Bus) -> u8;
    fn BCC(&mut self) -> u8;
    fn BCS(&mut self) -> u8;
    fn BEQ(&mut self) -> u8;
    fn BIT(&mut self, bus: &mut Bus) -> u8;
    fn BMI(&mut self) -> u8;
    fn BNE(&mut self) -> u8;
    fn BPL(&mut self) -> u8;
    fn BRK(&mut self, bus: &mut Bus) -> u8;
    fn BVC(&mut self) -> u8;
    fn BVS(&mut self) -> u8;
    fn CLC(&mut self) -> u8;
    fn CLD(&mut self) -> u8;
    fn CLI(&mut self) -> u8;
    fn CLV(&mut self) -> u8;
    fn CMP(&mut self, bus: &mut Bus) -> u8;
    fn CPX(&mut self, bus: &mut Bus) -> u8;
    fn CPY(&mut self, bus: &mut Bus) -> u8;
    fn DEC(&mut self, bus: &mut Bus) -> u8;
    fn DEX(&mut self) -> u8;
    fn DEY(&mut self) -> u8;
    fn EOR(&mut self, bus: &mut Bus) -> u8;
    fn INC(&mut self, bus: &mut Bus) -> u8;
    fn INX(&mut self) -> u8;
    fn INY(&mut self) -> u8;
    fn JMP(&mut self) -> u8;
    fn JSR(&mut self, bus: &mut Bus) -> u8;
    fn LDA(&mut self, bus: &mut Bus) -> u8;
    fn LDX(&mut self, bus: &mut Bus) -> u8;
    fn LDY(&mut self, bus: &mut Bus) -> u8;
    fn LSR(&mut self, bus: &mut Bus) -> u8;
    fn NOP(&mut self) -> u8;
    fn ORA(&mut self, bus: &mut Bus) -> u8;
    fn PHA(&mut self, bus: &mut Bus) -> u8;
    fn PHP(&mut self, bus: &mut Bus) -> u8;
    fn PLA(&mut self, bus: &mut Bus) -> u8;
    fn PLP(&mut self, bus: &mut Bus) -> u8;
    fn ROL(&mut self, bus: &mut Bus) -> u8;
    fn ROR(&mut self, bus: &mut Bus) -> u8;
    fn RTI(&mut self, bus: &mut Bus) -> u8;
    fn RTS(&mut self, bus: &mut Bus) -> u8;
    fn SBC(&mut self, bus: &mut Bus) -> u8;
    fn SEC(&mut self) -> u8;
    fn SED(&mut self) -> u8;
    fn SEI(&mut self) -> u8;
    fn STA(&mut self, bus: &mut Bus) -> u8;
    fn STX(&mut self, bus: &mut Bus) -> u8;
    fn STY(&mut self, bus: &mut Bus) -> u8;
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
        self.pc+=1;
        self.addr_abs = self.pc;
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

    fn ABS(&mut self, bus: &mut Bus) -> u8 {
        let lo: u16 = self.read(bus, self.pc, true).into();
        self.pc += 1;
        let hi: u16 = self.read(bus, self.pc, true).into();
        self.pc += 1;
        self.addr_abs = (hi << 8) | lo;
        0u8
    }
    fn ABX(&mut self, bus: &mut Bus) -> u8 {
        let lo: u16 = self.read(bus, self.pc, true).into();
        self.pc += 1;
        let hi: u16 = self.read(bus, self.pc, true).into();
        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.x as u16;

        match self.addr_abs | 0xFF00 != hi {
            false => 0u8,
            true => 1u8,
        }
    }
    fn ABY(&mut self, bus: &mut Bus) -> u8 {
        let lo: u16 = self.read(bus, self.pc, true).into();
        self.pc += 1;
        let hi: u16 = self.read(bus, self.pc, true).into();

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;

        match self.addr_abs | 0xFF00 != hi {
            false => 0u8,
            true => 1u8,
        }
    }
    fn IND(&mut self, bus: &mut Bus) -> u8 {
        let ptr_lo: u16 = self.read(bus, self.pc, true).into();
        self.pc += 1;
        let ptr_hi: u16 = self.read(bus, self.pc, true).into();
        self.pc += 1;
        let ptr = (ptr_hi << 8) | ptr_lo;
        self.addr_abs = match ptr_lo == 0x00FF {
            true => {
                ((self.read(bus, ptr & 0xFF00, true) as u16) << 8)
                    | self.read(bus, ptr, true) as u16
            }
            false => {
                ((self.read(bus, ptr + 1, true) as u16) << 8) | self.read(bus, ptr, true) as u16
            }
        };
        0u8
    }
    fn IZX(&mut self, bus: &mut Bus) -> u8 {
        let t: u16 = self.read(bus, self.pc, true).into();
        self.pc += 1;

        let lo: u16 = self.read(bus, (t + (self.x as u16)) & 0x00FF, true).into();
        let hi: u16 = self
            .read(bus, (t + ((self.x + 1) as u16)) & 0x00FF, true)
            .into();
        self.addr_abs = (hi << 8) | lo as u16;

        0u8
    }
    fn IZY(&mut self, bus: &mut Bus) -> u8 {
        let t: u16 = self.read(bus, self.pc, true).into();
        self.pc += 1;

        let lo: u16 = self.read(bus, t & 0x00FF, true).into();
        let hi: u16 = self.read(bus, (t + 1) & 0x00FF, true).into();
        self.addr_abs = ((hi << 8) | lo) + self.y as u16;

        match self.addr_abs & 0xFF00 != hi {
            false => 1u8,
            true => 0u8,
        }
    }
    fn REL(&mut self, bus: &mut Bus) -> u8 {
        self.addr_rel = self.read(bus, self.pc, true).into();
        self.pc += 1;
        if self.addr_rel & 0x80 != 0 {
            self.addr_rel |= 0xFF00;
        }

        0u8
    }
}

impl CpuApplyFunctions for OLC6502 {
    fn apply_op(&mut self, instruction: INSTRUCTION, bus: &mut Bus) -> u8 {
        match instruction.opcode.as_str() {
            "ADC" => self.ADC(bus),
            "AND" => self.AND(bus),
            "ASL" => self.ASL(bus),
            "BCC" => self.BCC(),
            "BCS" => self.BCS(),
            "BEQ" => self.BEQ(),
            "BIT" => self.BIT(bus),
            "BMI" => self.BMI(),
            "BNE" => self.BNE(),
            "BPL" => self.BPL(),
            "BRK" => self.BRK(bus),
            "BVC" => self.BVC(),
            "BVS" => self.BVS(),
            "CLC" => self.CLC(),
            "CLD" => self.CLD(),
            "CLI" => self.CLI(),
            "CLV" => self.CLV(),
            "CMP" => self.CMP(bus),
            "CPX" => self.CPX(bus),
            "CPY" => self.CPY(bus),
            "DEC" => self.DEC(bus),
            "DEX" => self.DEX(),
            "DEY" => self.DEY(),
            "EOR" => self.EOR(bus),
            "INC" => self.INC(bus),
            "INX" => self.INX(),
            "INY" => self.INY(),
            "JMP" => self.JMP(),
            "JSR" => self.JSR(bus),
            "LDA" => self.LDA(bus),
            "LDX" => self.LDX(bus),
            "LDY" => self.LDY(bus),
            "LSR" => self.LSR(bus),
            "NOP" => self.NOP(),
            "ORA" => self.ORA(bus),
            "PHA" => self.PHA(bus),
            "PHP" => self.PHP(bus),
            "PLA" => self.PLA(bus),
            "PLP" => self.PLP(bus),
            "ROL" => self.ROL(bus),
            "ROR" => self.ROR(bus),
            "RTI" => self.RTI(bus),
            "RTS" => self.RTS(bus),
            "SBC" => self.SBC(bus),
            "SEC" => self.SEC(),
            "SED" => self.SED(),
            "SEI" => self.SEI(),
            "STA" => self.STA(bus),
            "STX" => self.STX(bus),
            "STY" => self.STY(bus),
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
            "REL" => self.REL(bus),
            "ABS" => self.ABS(bus),
            "ABX" => self.ABX(bus),
            "ABY" => self.ABY(bus),
            "IND" => self.IND(bus),
            "IZX" => self.IZX(bus),
            "IZY" => self.IZY(bus),
            _ => 0u8,
        }
    }
}

impl OperationCodes for OLC6502 {
    /// Add with carry, Done 
    fn ADC(&mut self, bus: &mut Bus) -> u8 {
        self.fetch_data(bus);
        let tmp: u16 = (self.a + self.fetched_data + FLAGS6502::C as u8) as u16;
        self.set_flag(FLAGS6502::C, tmp > 255);
        self.set_flag(FLAGS6502::Z, tmp & 0x00FF == 0);
        self.set_flag(FLAGS6502::N, !tmp.get_low_byte().get_high_bit());
        self.set_flag(
            FLAGS6502::V,
            !(self.a ^ self.fetched_data) as u16 & (self.a as u16 ^ tmp) & 0x0080 != 0,
        );
        self.a = (tmp & 0x00FF) as u8;
        1u8
    }
    /// Bitwise AND, Done
    fn AND(&mut self, bus: &mut Bus) -> u8 {
        self.fetch_data(bus);
        self.a = self.a & self.fetched_data;
        self.set_flag(FLAGS6502::Z, self.a == 0x00);
        self.set_flag(FLAGS6502::N, !self.a.get_high_bit());
        1u8
    }
    /// Arithmetic Shift Left, Done
    fn ASL(&mut self, bus: &mut Bus) -> u8 {
        self.fetch_data(bus);
        self.a = self.fetched_data;
        self.set_flag(FLAGS6502::C,self.a.get_high_bit());
        self.a = self.a << 1;
        0u8
    }
    /// Branch on carry clear, Done
    fn BCC(&mut self) -> u8 {
        if self.get_flag(FLAGS6502::C) == 0 {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;
            if self.addr_abs & 0xFF00 != self.pc & 0xFF00 {
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }
        0u8
    }
    /// Branch on carry set, Done
    fn BCS(&mut self) -> u8 {
        if self.get_flag(FLAGS6502::C) == 1 {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;
            if self.addr_abs & 0xFF00 != self.pc & 0xFF00 {
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }
        0u8
    }
    /// Branch if equal, Done
    fn BEQ(&mut self) -> u8 {
        if self.get_flag(FLAGS6502::Z) == 1 {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;
            if self.addr_abs & 0xFF00 != self.pc & 0xFF00 {
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }
        0u8
    }
    /// Bit test, Done (by hand)
    fn BIT(&mut self, bus: &mut Bus) -> u8 {
        //TODO: Check if the opcode is the 89 version
        self.fetch_data(bus);
        let result = self.a & self.fetched_data;
        self.set_flag(FLAGS6502::Z, result == 0);
        self.set_flag(FLAGS6502::V, result.get_next_bit());
        self.set_flag(FLAGS6502::N, !result.get_high_bit());
        0u8
    }
    /// Branch if minus, Done
    fn BMI(&mut self) -> u8 {
        if self.get_flag(FLAGS6502::N) == 1 {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;
            if self.addr_abs & 0xFF00 != self.pc & 0xFF00 {
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }
        0u8
    }
    /// Branch not equal, Done
    fn BNE(&mut self) -> u8 {
        if self.get_flag(FLAGS6502::Z) == 0 {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;
            if self.addr_abs & 0xFF00 != self.pc & 0xFF00 {
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }
        0u8
    }
    /// Branch if positive, Done
    fn BPL(&mut self) -> u8 {
        if self.get_flag(FLAGS6502::N) == 0 {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;
            if self.addr_abs & 0xFF00 != self.pc & 0xFF00 {
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }
        0u8
    }
    /// Break, Done
    fn BRK(&mut self, bus: &mut Bus) -> u8 {
        self.set_flag(FLAGS6502::B, true);
        self.stkp = self.stkp.checked_add(1).unwrap_or(0);
        self.write(
            bus,
            0x0100 + self.stkp as u16,
            (self.pc >> 8 & 0x00FF) as u8,
        );
        self.stkp = self.stkp.checked_add(1).unwrap_or(0);
        self.write(bus, 0x0100 + self.stkp as u16, (self.pc & 0x00FF) as u8);
        self.stkp += 1;
        self.write(bus, 0x0100 + self.stkp as u16, self.status);
        self.addr_abs = 0xFFFE;
        let lo = self.read(bus, self.addr_abs + 0, true) as u16;
        let hi = self.read(bus, self.addr_abs + 1, true) as u16;
        self.pc = hi << 8 | lo;
        self.set_flag(FLAGS6502::B, true);
        0u8
    }
    /// Branch if overflow clear, Done
    fn BVC(&mut self) -> u8 {
        if self.get_flag(FLAGS6502::V) == 0 {
            self.cycles += 1;
            self.addr_abs = self.pc.checked_add(self.addr_rel).unwrap_or(self.addr_rel-self.pc);
            if self.addr_abs & 0xFF00 != self.pc & 0xFF00 {
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }
        0u8
    }
    /// Branch if overflow set, Done
    fn BVS(&mut self) -> u8 {
        if self.get_flag(FLAGS6502::V) == 1 {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;
            if self.addr_abs & 0xFF00 != self.pc & 0xFF00 {
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }
        0u8
    }
    /// Clear carry flag, Done
    fn CLC(&mut self) -> u8 {
        self.set_flag(FLAGS6502::C, false);
        0u8
    }
    /// Clear decimal mode, Done
    fn CLD(&mut self) -> u8 {
        self.set_flag(FLAGS6502::C, false);
        0u8
    }
    /// Clear interupt disabled, Done
    fn CLI(&mut self) -> u8 {
        self.set_flag(FLAGS6502::I, false);
        0u8
    }
    /// Clear overflow flag, Done
    fn CLV(&mut self) -> u8 {
        self.set_flag(FLAGS6502::V, false);
        0u8
    }
    /// Compare, Done
    fn CMP(&mut self, bus: &mut Bus) -> u8 {

        self.fetch_data(bus);
        let value = self.a - self.fetched_data;
        self.set_flag(FLAGS6502::N, !value.get_high_bit());
        self.set_flag(FLAGS6502::Z, self.a == value);
        self.set_flag(FLAGS6502::C, self.a >= value);
        1u8
    }
    /// Compare X register, Done
    fn CPX(&mut self, bus: &mut Bus) -> u8 {
        self.fetch_data(bus);
        let result = self.x - self.fetched_data;
        self.set_flag(FLAGS6502::N, !result.get_high_bit());
        self.set_flag(FLAGS6502::Z, result == 0);
        self.set_flag(FLAGS6502::C, self.x >= self.fetched_data);
        0u8
    }
    /// Compare Y register, Done
    fn CPY(&mut self, bus: &mut Bus) -> u8 {
        self.fetch_data(bus);
        let result = self.y.checked_sub(self.fetched_data).unwrap_or(self.fetched_data-self.y);
        
        self.set_flag(FLAGS6502::N, !result.get_high_bit());
        self.set_flag(FLAGS6502::Z, result == 0);
        self.set_flag(FLAGS6502::C, self.y >= self.fetched_data);
        0u8
    }
    /// Decrement value, Done
    fn DEC(&mut self, bus: &mut Bus) -> u8 {
        self.fetch_data(bus);
        let tmp = self.fetched_data - 1;
        self.write(bus, self.addr_rel, tmp);
        self.set_flag(FLAGS6502::Z, tmp == 0);
        self.set_flag(FLAGS6502::N, !tmp.get_high_bit());
        
        0u8
    }
    /// Decrement X register, Done
    fn DEX(&mut self) -> u8 {
        self.x -= 1;
        self.set_flag(FLAGS6502::Z, self.x==0);
        self.set_flag(FLAGS6502::N, !self.x.get_high_bit());
        0u8
    }
    /// Decrement Y register, Done
    fn DEY(&mut self) -> u8 {
        self.y -= 1;
        self.set_flag(FLAGS6502::Z, self.y==0);
        self.set_flag(FLAGS6502::N, !self.y.get_high_bit());
        0u8
    }
    /// Exclusive Or
    fn EOR(&mut self, bus: &mut Bus) -> u8 {
        self.fetch_data(bus);
        self.a ^= self.fetched_data;
        self.set_flag(FLAGS6502::N, !self.a.get_high_bit());
        self.set_flag(FLAGS6502::Z, self.a == 0);
        1u8
    }
    /// Increment data
    fn INC(&mut self, bus: &mut Bus) -> u8 {
        self.fetch_data(bus);
        let temp = self.fetched_data +1;
        self.write(bus, self.addr_abs, temp);
        self.set_flag(FLAGS6502::N, !temp.get_high_bit());
        self.set_flag(FLAGS6502::Z, temp == 0);
        0u8
    }
    /// Increment X register
    fn INX(&mut self) -> u8 {
        self.x += 1;
        self.set_flag(FLAGS6502::Z, self.x==0);
        self.set_flag(FLAGS6502::N, !self.x.get_high_bit());
        0u8
    }
    /// Increment Y register
    fn INY(&mut self) -> u8 {
        self.y += 1;
        self.set_flag(FLAGS6502::Z, self.y==0);
        self.set_flag(FLAGS6502::N, !self.y.get_high_bit());
        0u8
    }
    /// Jump to specified location
    fn JMP(&mut self) -> u8 {
        self.pc = self.addr_abs;
        0u8
    }
    /// Jump to sub routine, push current program counter to stack
    fn JSR(&mut self, bus: &mut Bus) -> u8 {
        self.pc -=1;
        self.write(bus, 0x0100 + self.stkp as u16, self.pc.get_high_byte());
        self.stkp-=1;
        self.write(bus, 0x0100 + self.stkp as u16, self.pc.get_high_byte());
        self.stkp-=1;
        self.pc = self.addr_abs;
        0u8
    }
    /// Load data to the accumumator
    fn LDA(&mut self, bus: &mut Bus) -> u8 {
        
        self.fetch_data(bus);
        
        self.a = self.fetched_data;
        self.set_flag(FLAGS6502::Z, self.a == 0);
        self.set_flag(FLAGS6502::N, !self.a.get_high_bit());
        1u8
    }
    /// Load data to X register
    fn LDX(&mut self, bus: &mut Bus) -> u8 {
        self.fetch_data(bus);
        self.x = self.fetched_data;
        self.set_flag(FLAGS6502::Z, self.x == 0);
        self.set_flag(FLAGS6502::N, !self.x.get_high_bit());
        1u8
    }
    /// Load data to Y register
    fn LDY(&mut self, bus: &mut Bus) -> u8 {
        self.fetch_data(bus);
        self.y = self.fetched_data;
        self.set_flag(FLAGS6502::Z, self.y == 0);
        self.set_flag(FLAGS6502::N, !self.y.get_high_bit());
        1u8
    }
    /// Logical shift right
    fn LSR(&mut self, bus: &mut Bus) -> u8{
        self.fetch_data(bus);
        let mut tmp = self.read(bus, self.addr_abs, true);
        self.set_flag(FLAGS6502::C, tmp.get_low_bit());
        tmp >>= 1;
        match self.lookup[self.curr_opcode as usize].addr_mode.as_str(){
            "IMP" => self.a = tmp,
            _ => self.write(bus, self.addr_abs, tmp)
        }
        
        self.set_flag(FLAGS6502::Z, tmp == 0);
        self.set_flag(FLAGS6502::N, !tmp.get_high_bit());
        0u8
    }
    /// No operation, do nothing 
    /// But there's a catch, some NOP needs more cycles (https://wiki.nesdev.com/w/index.php/CPU_unofficial_opcodes)
    fn NOP(&mut self) -> u8 {
        match self.curr_opcode{
            0xFC => 1u8,
            _ => 0u8
        }
    }
    /// Inclusive Or with accumulator
    fn ORA(&mut self, bus: &mut Bus) -> u8 {
        self.fetch_data(bus);
        self.a |= self.fetched_data;
        self.set_flag(FLAGS6502::Z, self.a == 0);
        self.set_flag(FLAGS6502::N, !self.a.get_high_bit());
        1u8
    }
    /// Push accumulator, Done
    fn PHA(&mut self, bus: &mut Bus) -> u8 {
        self.write(bus, 0x0100 + self.stkp as u16, self.a);
        self.stkp -= 1;
        0u8
    }
    /// Push status in the stack
    fn PHP(&mut self, bus: &mut Bus) -> u8 {
        self.write(bus, 0x0100 + self.stkp as u16, self.status | 0x10);
        self.stkp-=1;
        self.set_flag(FLAGS6502::B, false);
        self.set_flag(FLAGS6502::U, false);
        0u8
    }
    /// Pull accumulator, Done
    fn PLA(&mut self, bus: &mut Bus) -> u8 {
        self.stkp += 1;
        self.a = self.read(bus, 0x0100 + self.stkp as u16, true);
        self.set_flag(FLAGS6502::Z, self.a == 0);
        self.set_flag(FLAGS6502::N, !self.a.get_high_bit());
        0u8
    }
    // TODO: Check this one
    /// Pop status from the stack
    fn PLP(&mut self, bus: &mut Bus) -> u8 {
        self.stkp += 1;
        self.status = self.read(bus, 0x0100 + self.stkp as u16, true);
        0u8
    }
    /// Rotate on left
    fn ROL(&mut self, bus: &mut Bus) -> u8 {
        self.fetch_data(bus);
        let tmp = (self.fetched_data as u16) << 1 | self.get_flag(FLAGS6502::C) as u16;
        
        self.set_flag(FLAGS6502::C, tmp.get_nth_bit(8));
        self.set_flag(FLAGS6502::Z, tmp.get_low_byte() == 0 );
        self.set_flag(FLAGS6502::N, tmp.get_low_byte().get_high_bit());

        match self.lookup[self.curr_opcode as usize].addr_mode.as_str() {
            "IMP" => self.a = tmp.get_low_byte(),
            _ => self.write(bus, self.addr_abs, tmp.get_low_byte())
        }
        
        0u8
    }
    /// Rotate on right
    fn ROR(&mut self, bus: &mut Bus) -> u8 {

        self.fetch_data(bus);
        let mut tmp = self.fetched_data;
        self.set_flag(FLAGS6502::C, tmp.get_low_bit());
        tmp >>= 1;
        
        self.set_flag(FLAGS6502::Z, tmp == 0 );
        self.set_flag(FLAGS6502::N, tmp.get_high_bit());

        match self.lookup[self.curr_opcode as usize].addr_mode.as_str() {
            "IMP" => self.a = tmp,
            _ => self.write(bus, self.addr_abs, tmp)
        }
        
        0u8
    }
    /// Return from interupt, Done
    fn RTI(&mut self, bus: &mut Bus) -> u8 {
        self.stkp += 1;
        self.status = self.read(bus, 0x0100 + self.stkp as u16, true);
        self.status &= !(FLAGS6502::B as u8);
        self.status &= !(FLAGS6502::U as u8);
        self.stkp += 1;
        self.pc = self.read(bus, 0x0100 + self.stkp as u16, true) as u16;
        self.stkp += 1;
        self.pc |= (self.read(bus, 0x0100 + self.stkp as u16, true) as u16) << 8;
        0u8
    }
    /// Return from subroutine, Pop the program counter from the stack
    fn RTS(&mut self, bus: &mut Bus) -> u8 {
        self.stkp+=1;
        let hi = (self.read(bus, 0x0100 + self.stkp as u16, true) as u16) << 8;
        let lo = self.read(bus, 0x0100 + (self.stkp + 1) as u16, true) as u16;
        self.pc=hi+lo;

        0u8
    }
    /// Substract with carry, Done
    fn SBC(&mut self, bus: &mut Bus) -> u8 {
        self.fetch_data(bus);
        let value = self.fetched_data ^ 0x00FF;
        let tmp1 = self.a.add_overflow(value);
        let tmp = tmp1.add_overflow(FLAGS6502::C as u8) as u16;
        self.set_flag(FLAGS6502::C, tmp > 255);
        self.set_flag(FLAGS6502::Z, tmp & 0x00FF == 0);
        self.set_flag(FLAGS6502::N, tmp.get_low_byte().get_high_bit());
        self.set_flag(
            FLAGS6502::V,
            !(self.a ^ self.fetched_data) as u16 & (self.a as u16 ^ tmp) & 0x0080 != 0,
        );
        self.a = (tmp & 0x00FF) as u8;
        1u8
    }
    /// Set carry flag to 1
    fn SEC(&mut self) -> u8 {
        self.set_flag(FLAGS6502::C, true);
        0u8
    }
    /// Set Decimal to 1
    fn SED(&mut self) -> u8 {
        self.set_flag(FLAGS6502::D, true);
        0u8
    }
    /// Set disable interupt
    fn SEI(&mut self) -> u8 {
        self.set_flag(FLAGS6502::I, true);
        0u8
    }
    /// Store accumulator in memory
    fn STA(&mut self, bus: &mut Bus) -> u8 {
        self.write(bus, self.addr_abs, self.a);
        0u8
    }
    /// Store X register in memory
    fn STX(&mut self, bus: &mut Bus) -> u8 {
        self.write(bus, self.addr_abs, self.x);
        0u8
    }
    /// Store Y register in memory
    fn STY(&mut self, bus: &mut Bus) -> u8 {
        self.write(bus, self.addr_abs, self.y);
        0u8
    }
    /// Transfer Accumulator to X
    fn TAX(&mut self) -> u8 {
        self.x = self.a;
        self.set_flag(FLAGS6502::Z, self.x == 0);
        self.set_flag(FLAGS6502::N, self.x.get_high_bit());
        0u8
    }
    fn TAY(&mut self) -> u8 {
        self.y = self.a;
        self.set_flag(FLAGS6502::Z, self.y == 0);
        self.set_flag(FLAGS6502::N, self.y.get_high_bit());
        0u8
    }
    fn TSX(&mut self) -> u8 {
        self.x = self.stkp;
        self.set_flag(FLAGS6502::Z, self.x == 0);
        self.set_flag(FLAGS6502::N, self.x.get_high_bit());
        0u8
    }
    fn TXA(&mut self) -> u8 {
        self.a = self.x;
        self.set_flag(FLAGS6502::Z, self.a == 0);
        self.set_flag(FLAGS6502::N, self.a.get_high_bit());
        0u8
    }
    fn TXS(&mut self) -> u8 {
        self.stkp = self.x;
        0u8
    }
    fn TYA(&mut self) -> u8 {
        self.a = self.y;
        self.set_flag(FLAGS6502::Z, self.a == 0);
        self.set_flag(FLAGS6502::N, self.a.get_high_bit());
        0u8
    }

    fn XXX(&mut self) -> u8 {
        0u8
    }
}

impl CPUFunctions for OLC6502 {
    fn clock(&mut self, bus: &mut Bus) {
        if self.cycles == 0 {
            self.curr_opcode = self.read(bus, self.pc, true);
            
            self.set_flag(FLAGS6502::U, true);
            
            self.cycles = self.lookup[self.curr_opcode as usize].cycles;
            
            let additionnal_cycle_1 =
                self.apply_addressing_mode(self.lookup[self.curr_opcode as usize].clone(), bus);
            self.pc += 1;
            let additionnal_cycle_2 =
                self.apply_op(self.lookup[self.curr_opcode as usize].clone(), bus);
                
            self.cycles += additionnal_cycle_1 & additionnal_cycle_2;
            self.set_flag(FLAGS6502::U, true);
        }
        else{
            self.cycles -= 1;
        }
    }
    fn get_flag(&mut self, f: FLAGS6502) -> u8 {
        match (self.status & f as u8) > 0 {
            true => 1,
            false => 0,
        }
    }
    fn set_flag(&mut self, f: FLAGS6502, v: bool) {
        match v {
            true => self.status |= f as u8,
            false => self.status &= !(f as u8),
        }
    }
    fn reset(&mut self, bus: &mut Bus) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.stkp = 0xFD;
        self.status = 0x00 | FLAGS6502::U as u8;
        self.addr_abs = 0xFFFC;
        let lo = self.read(bus, self.addr_abs, true) as u16;
        let hi = self.read(bus, self.addr_abs + 1, true) as u16;
        self.pc = (hi << 8) | lo;
        self.addr_abs = 0;
        self.addr_rel = 0;
        self.fetched_data = 0;
        self.cycles = 8;
    }
    fn power(&mut self, bus: &mut Bus){
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.stkp = 0x00;
        self.status = 0x00 | FLAGS6502::U as u8;
        self.addr_abs = 0xFFFC;
        let lo = self.read(bus, self.addr_abs, true) as u16;
        let hi = self.read(bus, self.addr_abs + 1, true) as u16;
        self.pc = (hi << 8) | lo;
        
        self.addr_abs = 0;
        self.addr_rel = 0;
        self.fetched_data = 0;
        self.cycles = 3;
    }
    fn interupt_req(&mut self, bus: &mut Bus) {
        if self.get_flag(FLAGS6502::I) != 0 {
            self.write(
                bus,
                0x0100 + self.stkp as u16,
                ((self.pc >> 8) & 0x00FF) as u8,
            );
            self.stkp -= 1;
            self.write(bus, 0x0100 + self.stkp as u16, (self.pc & 0x00FF) as u8);
            self.stkp -= 1;
            self.set_flag(FLAGS6502::B, false);
            self.set_flag(FLAGS6502::U, true);
            self.set_flag(FLAGS6502::I, true);
            self.write(bus, 0x0100 + self.stkp as u16, self.status);
            self.stkp -= 1;
            self.addr_abs = 0xFFFE;
            let lo = self.read(bus, self.addr_abs + 0, true) as u16;
            let hi = self.read(bus, self.addr_abs + 1, true) as u16;
            self.pc = hi << 8 | lo;
            self.cycles = 7;
        }
    }
    fn fetch_data(&mut self, bus: &mut Bus) -> u8 {
        if self.lookup[self.curr_opcode as usize].addr_mode != "IMP" {
            self.fetched_data = self.read(bus, self.addr_abs, true);
        }
        self.fetched_data
    }
    fn non_maskable_interupt_req(&mut self, bus: &mut Bus) {
        self.write(
            bus,
            0x0100 + self.stkp as u16,
            ((self.pc >> 8) & 0x00FF) as u8,
        );
        self.stkp -= 1;
        self.write(bus, 0x0100 + self.stkp as u16, (self.pc & 0x00FF) as u8);
        self.stkp -= 1;
        self.set_flag(FLAGS6502::B, false);
        self.set_flag(FLAGS6502::U, true);
        self.set_flag(FLAGS6502::I, true);
        self.write(bus, 0x0100 + self.stkp as u16, self.status);
        self.stkp -= 1;
        self.addr_abs = 0xFFFA;
        let lo = self.read(bus, self.addr_abs + 0, true) as u16;
        let hi = self.read(bus, self.addr_abs + 1, true) as u16;
        self.pc = hi << 8 | lo;
        self.cycles = 8;
    }
}
