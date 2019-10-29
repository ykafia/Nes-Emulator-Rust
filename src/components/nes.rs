// use super::super::components::*;
// use super::super::utils::*;
pub struct NesData {
    pub ram: [u8; 64 * 1024],
}

pub trait DataActions {
    fn write(&mut self, addr: u16, data: u8);
    fn read(&mut self, addr: u16, read_only: bool) -> u8;
}

impl NesData {
    pub fn new() -> NesData {
        NesData {
            ram: [0u8; 64 * 1024],
        }
    }
}

impl DataActions for NesData {
    fn read(&mut self, addr: u16, read_only: bool) -> u8 {
        if addr < 0xFFFF {
            match read_only {
                true => self.ram[addr as usize],
                false => self.ram[addr as usize],
            }
        } else {
            0u8
        }
    }
    fn write(&mut self, addr: u16, data: u8) {
        if addr < 0xFFFF {
            self.ram[addr as usize] = data;
        }
    }
}
