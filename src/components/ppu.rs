// use super::super::utils::*;
// use super::*;
/// The Picture processing unit.
/// It should probably be handled by the computer itself depending the design.
/// This component handles the pictures drawn on the screen,
/// it has access to some shared rom from the cartridge and has its own ram components.
pub struct PPU {
    /// Ram data, from 0x2000 to 0x2FFF
    pub names: [[u8; 0x0400];2],
    /// Pattern data, from 0x0000 to 0x1FFF
    pub pattern: [u8; 0x2000],
    /// Pallette data, from 0x3000 to 0x3FFF
    pub pallette: [u8; 0x0020],
    /// Object attribute memory, should be shared with the cpu
    pub oam: [u8; 256],
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            pattern: [0u8; 0x2000],
            pallette: [0u8; 0x0020],
            oam: [0u8; 256],
            names : [[0u8; 0x0400];2]
        }
    }
}

impl Default for PPU {
    fn default() -> Self {
        Self::new()
    }
}

pub enum PPUComponents {
    NAMETABLES,
    PATTERN,
    PALLETTE,
}