use super::*;

/// The Picture processing unit.
/// It should probably be handled by the computer itself depending the design.
/// This component handles the pictures drawn on the screen,
/// it has access to some shared rom from the cartridge and has its own ram components.
/// The registers here are addressible in specific places for the CPU.
/// The CPU should be able to call the PPU by providing.
/// 
/// Pattern rom is supposed to happen in the cartridge as chr_rom
pub struct PPU {
    /// Ram data, from 0x2000 to 0x2FFF
    pub name_table: [u8; 0x1000],
    /// Pallette data, from 0x3000 to 0x3FFF
    pub pallette: [u8; 0x0100],
    /// Object attribute memory, should be shared with the cpu
    pub oam: [u8; 256],

    /// [0x2000] \ Control register
    pub ctrl: u8,
    /// [0x2001] \ Mask register
    pub mask: u8,
    /// [0x2002] \ Status register
    pub status: u8,
    /// [0x2003] \ OAM Read/Write address register
    pub oam_addr: u8,
    /// [0x2004] \ OAM Read/Write data register
    pub oam_data: u8,
    /// [0x2005] \ Fine scroll position register (two writes: X scroll, Y scroll)
    pub scroll: u8,
    /// [0x2006] \ PPU Read/Write address register
    pub addr: u8,
    /// [0x2007] \ PPU data Read/Write register
    pub data: u8,
    /// [0x4014] \ OAM High adress register
    pub oam_dma: u8,
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            name_table: [0u8; 0x1000],
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
    pub fn clock(&mut self) {}
    pub fn write(&mut self, nes : &mut NesData, addr: u16, data: u8) {
        match addr.to_where() {
            PPUComponents::PATTERN => self.chr_write(nes,addr, data),
            PPUComponents::PALLETTE => self.pallette[addr.to_component_data()] = data,
            PPUComponents::NAMETABLE => self.name_table[addr.to_component_data()] = data,
            PPUComponents::NTMIRROR => self.name_table[addr.to_component_data()] = data,
        }
    }
    fn read(&self, addr: u16, nes : &NesData, read_only: bool) -> u8 {
        match addr.to_where() {
            PPUComponents::PALLETTE => self.pallette[addr.to_component_data()],
            PPUComponents::PATTERN => self.chr_read(nes,addr,read_only),
            PPUComponents::NAMETABLE => self.name_table[addr.to_component_data()],
            PPUComponents::NTMIRROR => self.name_table[addr.to_component_data()],
        }
    }
    pub fn cpu_read(&self, addr:u16, read_only: bool) -> u8 {
        match addr &0x7{
            0x0 => self.ctrl,
            0x1 => self.mask,
            0x2 => self.status,
            0x3 => self.oam_addr,
            0x4 => self.oam_data,
            0x5 => self.scroll,
            0x6 => self.addr,
            0x7 => self.data,
            _ => 0u8
        }
    }
    pub fn cpu_write(&mut self, addr:u16, data : u8){
        match addr &0x7{
            0x0 => self.ctrl = data,
            0x1 => self.mask = data,
            0x2 => self.status = data,
            0x3 => self.oam_addr = data,
            0x4 => self.oam_data = data,
            0x5 => self.scroll = data,
            0x6 => self.addr = data,
            0x7 => self.data = data,
            _ => ()
        }
    }
    fn get_control_flag(&mut self, f: PPUCTRL) -> u8 {
        match (self.ctrl & f.bits) > 0 {
            true => 1,
            false => 0,
        }
    }
    fn set_control_flag(&mut self, f: PPUCTRL, v: bool) {
        match v {
            true => self.ctrl |= f.bits,
            false => self.ctrl &= !(f.bits),
        }
    }
    fn get_status_flag(&mut self, f: PPUSTATUS) -> u8 {
        match (self.status & f.bits) > 0 {
            true => 1,
            false => 0,
        }
    }
    fn set_status_flag(&mut self, f: PPUSTATUS, v: bool) {
        match v {
            true => self.status |= f.bits,
            false => self.status &= !(f.bits),
        }
    }
    fn get_mask_flag(&mut self, f: PPUMASK) -> u8 {
        match (self.mask & f.bits) > 0 {
            true => 1,
            false => 0,
        }
    }
    fn set_mask_flag(&mut self, f: PPUMASK, v: bool) {
        match v {
            true => self.mask |= f.bits,
            false => self.mask &= !(f.bits),
        }
    }
    /// Reads from the NES common data such as rom and patterns.
    pub fn chr_read(&self, nes: &NesData, addr: u16, read_only: bool) -> u8 {
        nes.ppu_read(addr, read_only)
    }
    pub fn chr_write(&self, nes: &mut NesData, addr: u16, data: u8) {
        nes.ppu_write(addr, data)
    }
}



pub enum PPUComponents {
    /// Pattern table Contains the sprites, usually connected to the ROM
    PATTERN,
    /// Considered as VRAM, can be mapped and extended by a ROM
    NAMETABLE,
    /// Nametable 0 mirror from 0x2000-0x2EFF, mostly unused address range
    NTMIRROR,
    PALLETTE,
}

impl AddrWhere<PPUComponents> for u16 {
    fn to_where(&self) -> PPUComponents {
        let x = *self;
        if x < 0x2000 {
            PPUComponents::PATTERN
        } else if x >= 0x2000 && x < 0x3000 {
            PPUComponents::NAMETABLE
        } else if x >= 0x3000 && x < 0x3EFF {
            PPUComponents::NTMIRROR
        } else {
            PPUComponents::PALLETTE
        }
    }
}


trait AddrConvert<Component> {
    fn to_component_data(&self) -> usize;
}

impl AddrConvert<PPUComponents> for u16
where
    u16: AddrWhere<PPUComponents>,
{
    fn to_component_data(&self) -> usize {
        match self.to_where() {
            PPUComponents::PATTERN => self.clone() as usize,
            PPUComponents::NAMETABLE => (self - 0x2000) as usize,
            PPUComponents::NTMIRROR => (self - 0x2000) as usize,
            PPUComponents::PALLETTE => (self - 0x3000) as usize,
        }
    }
}
#[allow(dead_code)]
enum PPUFLAGS {
    CONTROL,
    MASK,
    STATUS,
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
