// use super::super::components::*;
// use super::super::utils::*;
pub struct Bus {
    pub ram: [u8; 64 * 1024],
}

pub trait BusActions {
    fn write(&mut self, addr: u16, data: u8);
    fn read(&mut self, addr: u16, read_only: bool) -> u8;
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: [0u8; 64 * 1024],
        }
    }
}

impl BusActions for Bus {
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
