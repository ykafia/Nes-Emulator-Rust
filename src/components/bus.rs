use super::super::components::*;
use super::super::utils::*;

pub struct Bus {
    pub ram: [u8; 64 * 1024],
    pub cpu: OLC6502,
}

pub trait BusActions {
    fn write(&mut self, addr: usize, data: u8);
    fn read(&mut self, addr: usize, read_only: bool) -> u8;
}

impl Bus {
    fn new() -> Bus {
        Bus {
            ram: [0u8; 64 * 1024],
            cpu: OLC6502::new(),
        }
    }
}

impl BusActions for Bus {
    fn read(&mut self, addr: usize, read_only: bool) -> u8 {
        if addr > 0x0000 && addr < 0xFFFF {
            match read_only {
                true => self.ram[addr],
                false => self.ram[addr],
            }
        } else {
            0u8
        }
    }
    fn write(&mut self, addr: usize, data: u8) {
        if addr > 0x0000 && addr < 0xFFFF {
            self.ram[addr] = data;
        }
    }
}
