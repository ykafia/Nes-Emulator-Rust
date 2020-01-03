use super::utils::*;



pub struct NesData {
    /// Ram data, from 0x0000 to 0x1FFF
    pub ram: [u8; 0x2000],
    /// Cartridge data, from 0x4020 to 0xFFFF
    pub cartridge: Cartridge,
    /// Counts the number of time a clock was called
    pub system_counter : u128,
}



impl NesData {
    pub fn new() -> NesData {
        NesData {
            ram: [0u8; 0x2000],
            cartridge: Cartridge::new(),
            system_counter : 0
        }
    }
    pub fn clock(&mut self, cpu : &mut CPU6502, ppu : &mut PPU) {
        cpu.clock(self);
        for _ in 0..3 {
            ppu.clock();
        }
        self.system_counter += 1;
    }
    /// Insert memory in the nes game in the cartridge data space.
    pub fn insert_cartridge(&mut self, pathfile : &str) {
        self.cartridge.load(pathfile);
        
    }
    /// Reset the whole memory around.
    pub fn reset_memory(&mut self) {
        self.ram = [0u8;0x2000];
        self.cartridge = Cartridge::new();
    }
    pub fn read(&self, addr: u16, read_only: bool, ppu : Option<PPU>) -> u8 {
        match addr.to_where() {
            NESComponents::RAM => match read_only {
                true => self.ram[(addr % 0x07ff) as usize],
                false => self.ram[(addr % 0x07ff) as usize],
            },
            NESComponents::CARTRIDGE => match read_only {
                true => self.cartridge[(addr - 0x4020) as usize],
                false => self.cartridge[(addr - 0x4020) as usize],
            },
            NESComponents::PPU => {
                match ppu {
                    Some(mut x) => {
                        x.cpu_read(addr,read_only)
                    },
                    _ => {
                        panic!("No ppu given")
                    }
                }
            }
            _ => 0u8,
        }
    }
    pub fn write(&mut self, addr: u16, data: u8, ppu : Option<PPU>) {
        match addr.to_where() {
            NESComponents::RAM => self.ram[(addr % 0x07ff) as usize] = data,
            NESComponents::PPU => {
                match ppu {
                    //TODO: Should ppu write work only on its registers?
                    Some(mut x) => x.cpu_write(addr & 0x7,data),
                    None => panic!("No PPU given")
                }
            }
            NESComponents::CARTRIDGE => self.cartridge[(addr - 0x4020) as usize] = data,
            _ => (),
        }
    }
}


enum NESComponents {
    APU,
    APUDISABLED,
    PPU,
    RAM,
    CARTRIDGE,
    NOCOMP,
}


/// Helper to dispatch an address to a component/register
impl AddrWhere<NESComponents> for u16 {
    fn to_where(&self) -> NESComponents {
        let x = *self;
        if x < 0x2000 {
            NESComponents::RAM
        } else if x >= 0x2000 && x < 4000 {
            // Adresses are mirrors of the PPU registers addresses from 2000 to 2007 (repeats every 8 bytes)
            NESComponents::PPU
        } else if x >= 0x4000 && x < 0x4018 {
            NESComponents::APU
        } else if x >= 0x4018 && x < 0x4020 {
            // Some apu calls that are normally disabled in the NES
            NESComponents::APUDISABLED
        } else if x >= 0x4020 && x < 0xFFFF {
            NESComponents::CARTRIDGE
        } else {
            NESComponents::NOCOMP
        }
    }
}


// TODO: find a way for DRY coding

/// Trait that converts an address to the position it should have in the array
/// of its struct
/// This trait was rewritten to keep it enclosed in this crate.
trait AddrConvert<Component> {
    fn to_comp_data(&self) -> usize;
}
impl AddrConvert<NESComponents> for u16 where u16 : AddrWhere<NESComponents> {
    fn to_comp_data(&self) -> usize {
        match self.to_where() {
            NESComponents::RAM => (self + 0) as usize,
            _ => (self + 0) as usize
        }
    }
}