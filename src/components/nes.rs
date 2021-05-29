// use super::super::components::*;
use super::{super::utils::*, APU, Cartridge, PPU, PPUComponents};

pub struct NesData {
    /// Ram data, from 0x0000 to 0x1FFF
    pub ram: [u8; 0x2000],
    /// Cartridge data, from 0x4020 to 0xFFFF
    pub cartridge: Cartridge,
    /// PPU Registers
    pub ppu_registers : [u8; 0x0007],
    /// APU Registers
    pub apu : APU,
    /// PPU struct containing the ppu data
    pub ppu : PPU,
    /// Clock counter
    pub clock_counter : u128,
}

pub trait DataActions {
    fn cpu_write(&mut self, addr: u16, data: u8);
    fn cpu_read(&mut self, addr: u16, read_only: bool) -> u8;
    fn ppu_write(&mut self, addr: u16, data: u8);
    fn ppu_read(&mut self, addr: u16, read_only: bool) -> u8;
}

impl NesData {
    pub fn new() -> NesData {
        NesData {
            ram: [0u8; 0x2000],
            cartridge: Cartridge::new(),
            ppu_registers : [0u8; 0x0007],
            apu : APU::new(),
            ppu : PPU::new(),
            clock_counter: 0,
        }
    }
    pub fn insert_cartridge(&mut self, cartridge: [u8; 0xBFDF]) {
        self.cartridge.load(cartridge);
    }
    pub fn reset_memory(&mut self) {}
    pub fn clock(&mut self) {}
}

impl DataActions for NesData {
    fn cpu_write(&mut self, addr: u16, data: u8) {
        match addr.to_where() {
            NESComponents::RAM => self.ram[(addr % 0x07ff) as usize] = data,
            NESComponents::APU => self.apu.registers[((addr - 0x4000) & 0x0017) as usize] = data,
            NESComponents::PPU => self.ppu_registers[((addr - 0x2000) & 0x0007) as usize] = data,
            NESComponents::CARTRIDGE => self.cartridge[(addr - 0x4020) as usize] = data,
            _ => (),
        }
    }
    fn cpu_read(&mut self, addr: u16, read_only: bool) -> u8 {
        match addr.to_where() {
            NESComponents::RAM => match read_only {
                true => self.ram[(addr & 0x07ff) as usize],
                false => self.ram[(addr & 0x07ff) as usize],
            },
            NESComponents::CARTRIDGE => match read_only {
                true => self.cartridge[(addr - 0x4020) as usize],
                false => self.cartridge[(addr - 0x4020) as usize],
            },
            NESComponents::PPU => match read_only {
                true => self.ppu_registers[((addr - 0x2000) & 0x0007) as usize],
                false => self.ppu_registers[((addr - 0x2000) & 0x0007) as usize],
            },
            NESComponents::APU => match read_only {
                true => self.apu.registers[((addr - 0x4000) & 0x0017) as usize],
                false => self.apu.registers[((addr - 0x4000) & 0x0017) as usize],
            },
            _ => 0u8,
        }
    }

    fn ppu_write(&mut self, addr: u16, data: u8) {
        match addr.to_where() {
            PPUComponents::PALLETTE => self.ppu.pallette[addr as usize] = data,
            PPUComponents::PATTERN => self.ppu.pattern[addr as usize] = data, // TODO: Should it write on the ROM?
            PPUComponents::NAMETABLES => {
                let index = if (addr - 0x2000) & 0x0800 < 0x400 {0} else {1};
                self.ppu.names[index][addr as usize] = data;
            },
        }
    }

    fn ppu_read(&mut self, addr: u16, read_only: bool) -> u8 {
        match addr.to_where() {
            PPUComponents::PALLETTE => self.ppu.pallette[addr as usize],
            PPUComponents::PATTERN => self.cartridge[(addr + 0x4020) as usize],
            PPUComponents::NAMETABLES => {
                let index = if (addr - 0x2000) & 0x0800 < 0x400 {0} else {1};
                self.ppu.names[index][((addr - 0x2000) & 0x0400) as usize]
            },
        }
    }
}

enum NESComponents {
    RAM,
    APU,
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
        } else if x >= 0x4000 && x < 0x4017 {
            NESComponents::APU 
        } else if x >= 0x4020 && x < 0xFFFF {
            NESComponents::CARTRIDGE
        } else {
            NESComponents::NOCOMP
        }
    }
}
impl AddrConvert<PPUComponents> for u16 {
    fn to_where(&self) -> PPUComponents {
        let x = *self;
        if x < 0x2000 {
            PPUComponents::PATTERN
        } else if x >= 0x2000 && x < 0x3F00 {
            PPUComponents::NAMETABLES
        } else {
            PPUComponents::PALLETTE
        }
    }
}
