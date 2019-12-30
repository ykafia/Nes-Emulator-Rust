// use super::super::components::*;
use super::super::utils::*;
use super::super::*;
pub struct NesData {
    /// Ram data, from 0x0000 to 0x1FFF
    pub ram: [u8; 0x2000],
    /// Cartridge data, from 0x4020 to 0xFFFF
    pub cartridge: [u8; 0xBFDF],
}

pub trait DataActions {
    fn write(&mut self, addr: u16, data: u8);
    fn read(&self, addr: u16, read_only: bool, ppu : Option<PPU>) -> u8;
}

impl NesData {
    pub fn new() -> NesData {
        NesData {
            ram: [0u8; 0x2000],
            cartridge: [0u8; 0xBFDF],
        }
    }
    /// Insert memory in the nes game in the cartridge data space.
    pub fn insert_cartridge(&mut self, cartr: [u8; 0xBFDF]) {
        self.cartridge = cartr;
    }
    /// Reset the whole memory around.
    pub fn reset_memory(&mut self) {
        self.ram = [0u8;0x2000];
        self.cartridge = [0u8;0xBFDF];
    }
}

impl DataActions for NesData {
    fn read(&self, addr: u16, read_only: bool, ppu : Option<PPU>) -> u8 {
        match addr.to_where() {
            NESComponents::RAM => match read_only {
                true => self.ram[(addr % 0x07ff) as usize],
                false => self.ram[(addr % 0x07ff) as usize],
            },
            NESComponents::CARTRIDGE => match read_only {
                true => self.cartridge[(addr - 0x4020) as usize],
                false => self.cartridge[(addr - 0x4020) as usize],
            },
            NESComponents::PPU =>
            {
                match ppu {
                    Some(x) => {
                        x.ppu_read(addr,read_only)
                    },
                    _ => {
                        println!("No ppu given");
                        0
                    }
                }
            }
            _ => 0u8,
        }
    }
    fn write(&mut self, addr: u16, data: u8) {
        match addr.to_where() {
            NESComponents::RAM => self.ram[(addr % 0x07ff) as usize] = data,

            NESComponents::CARTRIDGE => self.cartridge[(addr - 0x4020) as usize] = data,
            _ => (),
        }
    }
}

enum NESComponents {
    RAM,
    CARTRIDGE,
    PPU,
    NOCOMP,
}

impl AddrConvert<NESComponents> for u16 {
    fn to_where(&self) -> NESComponents {
        let x = *self;
        if x < 0x2000 {
            NESComponents::RAM
        } else if x >= 0x2000 && x < 0x3FFF {
            NESComponents::PPU
        } else if x >= 0x4020 && x < 0xFFFF {
            NESComponents::CARTRIDGE
        } else if x >= 0x2000 && x < 0x3FFF {
            NESComponents::PPU
        } else {
            NESComponents::NOCOMP
        }
    }
}
