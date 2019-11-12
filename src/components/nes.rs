// use super::super::components::*;
use super::super::utils::*;
pub struct NesData {
    /// Ram data, from 0x0000 to 0x1FFF
    pub ram: [u8; 0x2000],
    /// Cartridge data, from 0x4020 to 0xFFFF
    pub cartridge : [u8;0xBFDF]
}



pub trait DataActions {
    fn write(&mut self, addr: u16, data: u8);
    fn read(&mut self, addr: u16, read_only: bool) -> u8;
}

impl NesData {
    pub fn new() -> NesData {
        NesData {
            ram: [0u8; 0x2000],
            cartridge : [0u8; 0xBFDF]
        }
    }
}

impl DataActions for NesData {
    fn read(&mut self, addr: u16, read_only: bool) -> u8 {
        match addr.to_where() {
            NESComponents::RAM=>
                match read_only{
                    true => self.ram[(addr % 0x07ff) as usize],
                    false => self.ram[(addr % 0x07ff) as usize]
                },
            NESComponents::CARTRIDGE=>
                match read_only{
                    true => self.cartridge[addr as usize],
                    false => self.cartridge[addr as usize]
                },
            NESComponents::PPU => 
                match read_only{
                    true => self.cartridge[addr as usize],
                    false => self.cartridge[addr as usize]
                },
            _ => 0u8
        }
    }
    fn write(&mut self, addr: u16, data: u8) {
        match addr.to_where() {
            NESComponents::RAM=>
                self.ram[(addr % 0x07ff) as usize] = data,
                    
            NESComponents::CARTRIDGE=>
                self.cartridge[addr as usize] = data,
            _ => ()
        }
    }
}

enum NESComponents {
    RAM,
    CARTRIDGE,
    PPU,
    NOCOMP
}

impl AddrConvert<NESComponents> for u16 {
    fn to_where(&self) -> NESComponents{
        let x = *self;
        if x < 0x2000 {
            NESComponents::RAM
        }
        else if x >= 0x4020 && x < 0xFFFF{
            NESComponents::CARTRIDGE
        }
        else if x>= 0x2000 && x<0x3FFF{
            NESComponents::PPU
        }
        else {
            NESComponents::NOCOMP
        }
        
    }
}
