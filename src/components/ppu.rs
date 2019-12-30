use super::super::utils::*;
use super::*;


/// The Picture processing unit.
/// It should probably be handled by the computer itself depending the design.
/// This component handles the pictures drawn on the screen,
/// it has access to some shared rom from the cartridge and has its own ram components.
pub struct PPU {
    /// Ram data, from 0x2000 to 0x2FFF
    pub ram: [u8; 0x1000],
    /// Pattern data, from 0x0000 to 0x1FFF
    pub pattern: [u8; 0x2000],
    /// Pallette data, from 0x3000 to 0x3FFF
    pub pallette: [u8; 0x0100],
    /// Object attribute memory, should be shared with the cpu
    pub OAM: [u8; 256],
    
}




impl PPU {
    pub fn new() -> PPU {
        PPU {
            ram: [0u8; 0x1000],
            pattern: [0u8; 0x2000],
            pallette: [0u8; 0x0100],
            OAM: [0u8; 256],
        }
    }
    pub fn ppu_write(&mut self, addr: u16, data: u8) {
        match addr.to_where() {
            PPUComponents::PALLETTE => self.pallette[addr as usize] = data,
            PPUComponents::PATTERN => self.pattern[addr as usize] = data,
            PPUComponents::RAM => self.ram[addr as usize] = data,
        }
    }
    pub fn ppu_read(&self, addr: u16, read_only: bool) -> u8 {
        match addr.to_where() {
            // TODO: do the pattern read from the cartridge.
            PPUComponents::PALLETTE => self.pallette[addr as usize],
            PPUComponents::PATTERN => self.pattern[addr as usize],
            PPUComponents::RAM => self.ram[addr as usize],
        }
    }
    #[warn(dead_code)]
    fn cpu_read(&mut self, cpu: &mut CPU6502, nes: &mut NesData, addr: u16, read_only: bool) -> u8 {
        cpu.read(nes, addr, true)
    }
    #[warn(dead_code)]
    fn cpu_write(&mut self, cpu: &mut CPU6502, nes: &mut NesData, addr: u16, data: u8) {
        cpu.write(nes, addr, data);
    }
}

enum PPUComponents {
    RAM,
    PATTERN,
    PALLETTE,
}

impl AddrConvert<PPUComponents> for u16 {
    fn to_where(&self) -> PPUComponents {
        let x = *self;
        if x < 0x2000 {
            PPUComponents::PATTERN
        } else if x >= 0x2000 && x < 0x3000 {
            PPUComponents::RAM
        } else {
            PPUComponents::PALLETTE
        }
    }
}


bitflags! {
    /// PPU Controler flag
    /// Various flags controlling PPU operation
    pub struct PPUCTRL : u8 {
        /// Base nametable address
        /// (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
        const N = 0b0000_0011;
        /// VRAM address increment per CPU read/write of PPUDATA
        /// (0: add 1, going across; 1: add 32, going down)
        const I = 0b0000_0100;
        /// Sprite pattern table address for 8x8 sprites
        /// (0: $0000; 1: $1000; ignored in 8x16 mode)
        const S = 0b0000_1000;
        /// Background pattern table address (0: $0000; 1: $1000)
        const B = 0b0001_0000;
        /// Sprite size (0: 8x8 pixels; 1: 8x16 pixels)
        const H = 0b0010_0000;
        /// PPU master/slave select
        /// (0: read backdrop from EXT pins; 1: output color on EXT pins)
        const P = 0b0100_0000;
        /// Generate an NMI at the start of the vertical blanking interval (0: off; 1: on)
        const V = 0b1000_0000;
    }
}
bitflags! {
    /// Flag for the PPU MASK
    pub struct PPUMASK : u8 {
        /// Greyscale (0: normal color, 1: produce a greyscale display)
        const GRAY = 0b0000_0001;
        /// 1: Show background in leftmost 8 pixels of screen, 0: Hide
        const BGL = 0b0000_0010;
        /// 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
        const SPL = 0b0000_0100;
        /// 1: Show background
        const BG = 0b0000_1000;
        /// 1: Show sprites
        const SP = 0b0001_0000;
        /// Emphasize red
        const R = 0b0010_0000;
        /// Emphasize green
        const G = 0b0100_0000;
        /// Emphasize blue
        const B = 0b1000_0000;
    }
}

bitflags! {
    /// Flag for the PPU Status
    pub struct PPUSTATUS : u8 {
        /// Least significant bits previously written into a PPU register 
        /// (due to register not being updated for this address)
        const Z = 0b0001_1111;
        /// Emphasize green
        const O = 0b0010_0000;
        /// Emphasize blue
        const S = 0b0100_0000;
        const V = 0b1000_0000;
    }
}