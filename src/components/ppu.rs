
use super::super::utils::*;
/// The Picture processing unit. 
/// It should probably be handled by the computer itself depending the design.
/// This component handles the pictures drawn on the screen, 
/// it has access to some shared rom from the cartridge and has its own ram components.
pub struct PPU{
    /// Ram data, from 0x2000 to 0x2FFF 
    pub ram : [u8;0x1000],
    /// Pattern data, from 0x0000 to 0x1FFF
    pub pattern : [u8;0x2000],
    /// Pallette data, from 0x3000 to 0x3FFF 
    pub pallette : [u8;0x0100],
    /// Object attribute memory, should be shared with the cpu
    pub OAM : [u8;256]
}

impl PPU {
    fn new() -> PPU {
        PPU {
            ram : [0u8;0x1000],
            pattern : [0u8;0x2000],
            pallette : [0u8;0x0100],
            OAM : [0u8;256]
        }
    }
    fn write(&mut self, addr : u16, data:u8) {
        match addr.to_where() {
            PPUComponents::PALLETTE => 
                self.pallette[addr as usize] = data,
            PPUComponents::PATTERN =>
                self.pattern[addr as usize] = data,
            PPUComponents::RAM =>
                self.ram[addr as usize] = data,
        }
    }
    fn read(&mut self, addr : u16, read_only : bool) -> u8{
        match addr.to_where() {
            PPUComponents::PALLETTE => 
                self.pallette[addr as usize],
            PPUComponents::PATTERN =>
                self.pattern[addr as usize],
            PPUComponents::RAM =>
                self.ram[addr as usize]
        }
    }
}

enum PPUComponents{
    RAM,
    PATTERN,
    PALLETTE
}

impl AddrConvert<PPUComponents> for u16 {
    fn to_where(&self) -> PPUComponents{
        let x = *self;
        if x < 0x2000 {
            PPUComponents::PATTERN
        }
        else if x >= 0x2000 && x < 0x3000{
            PPUComponents::RAM
        }
        else {
            PPUComponents::PALLETTE
        }
    }
}