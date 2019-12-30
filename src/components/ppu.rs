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
    pub oam: [u8; 256],

    /// Control register, address : 0x2000
    pub ctrl: u8,
    /// Mask register, address : 0x2001
    pub mask: u8,
    /// Status register, address : 0x2002
    pub status: u8,
    /// OAM Read/Write address register, address : 0x2003
    pub oam_addr: u8,
    /// OAM Read/Write data register, address : 0x2004
    pub oam_data: u8,
    /// Fine scroll position register (two writes: X scroll, Y scroll), address : 0x2005
    pub scroll: u8,
    /// PPU Read/Write address register, address : 0x2006
    pub addr: u8,
    /// PPU data Read/Write register, address : 0x2007
    pub data: u8,
    /// OAM High adress register, address : 0x4014
    pub oam_dma: u8,
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            ram: [0u8; 0x1000],
            pattern: [0u8; 0x2000],
            pallette: [0u8; 0x0100],
            oam: [0u8; 256],
            addr: 0,
            ctrl: 0,
            data: 0,
            mask: 0,
            oam_addr: 0,
            oam_dma: 0,
            oam_data: 0,
            scroll: 0,
            status: 0,
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
        const N = 3;
        /// VRAM address increment per CPU read/write of PPUDATA
        /// (0: add 1, going across; 1: add 32, going down)
        const I = 1<<2;
        /// Sprite pattern table address for 8x8 sprites
        /// (0: $0000; 1: $1000; ignored in 8x16 mode)
        const S = 1<<3;
        /// Background pattern table address (0: $0000; 1: $1000)
        const B = 1<<4;
        /// Sprite size (0: 8x8 pixels; 1: 8x16 pixels)
        const H = 1<<5;
        /// PPU master/slave select
        /// (0: read backdrop from EXT pins; 1: output color on EXT pins)
        const P = 1<<6;
        /// Generate an NMI at the start of the vertical blanking interval (0: off; 1: on)
        const V = 1<<7;
    }
}
bitflags! {
    /// Flag for the PPU MASK
    pub struct PPUMASK : u8 {
        /// Greyscale (0: normal color, 1: produce a greyscale display)
        const GRAY = 1;
        /// 1: Show background in leftmost 8 pixels of screen, 0: Hide
        const BGL = 1<<1;
        /// 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
        const SPL = 1<<2;
        /// 1: Show background
        const BG = 1<<3;
        /// 1: Show sprites
        const SP = 1<<4;
        /// Emphasize red
        const R = 1<<5;
        /// Emphasize green
        const G = 1<<6;
        /// Emphasize blue
        const B = 1<<7;
        const RGB = Self::R.bits | Self::G.bits | Self::B.bits;
    }
}

bitflags! {
    /// Flag for the PPU Status
    pub struct PPUSTATUS : u8 {
        /// Sprite overflow. The intent was for this flag to be set
        /// whenever more than eight sprites appear on a scanline, but a
        /// hardware bug causes the actual behavior to be more complicated
        /// and generate false positives as well as false negatives; see
        /// PPU sprite evaluation. This flag is set during sprite
        /// pre-render line.
        const O = 1<<5;
        /// Sprite 0 Hit.  Set when a nonzero pixel of sprite 0 overlaps
        /// a nonzero background pixel; cleared at dot 1 of the pre-render
        /// line.  Used for raster timing.
        const S = 1<<6;
        /// Vertical blank has started (0: not in vblank; 1: in vblank).
        /// Set at dot 1 of line 241 (the line *after* the post-render
        /// line); cleared after reading $2002 and at dot 1 of the
        /// pre-render line.
        const V = 1<<7;
        /// Least significant bits previously written into a PPU register
        /// (due to register not being updated for this address)
        const UNUSED = !( Self::O.bits | Self::S.bits | Self::V.bits );
    }
}
