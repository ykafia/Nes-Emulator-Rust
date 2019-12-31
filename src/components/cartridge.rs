use std::ops::{Index,IndexMut};
use std::fs::File;
use std::io::prelude::*;
use super::*;

pub struct Cartridge {
    pub data : Vec<u8>,
    pub program_rom : Vec<u8>,
    pub character_rom : Vec<u8>,
    pub mapper_id : u8,
    pub prgm_bank_n : u8,
    pub chr_bank_n : u8
}

impl Cartridge {
    pub fn new() -> Cartridge {
        Cartridge {
            data : Vec::new(),
            program_rom : Vec::new(),
            character_rom : Vec::new(),
            mapper_id : 0,
            prgm_bank_n : 0,
            chr_bank_n : 0,
        }
    }
    pub fn load(&mut self, pathfile:&str) {
        let mut file = File::open(pathfile).unwrap();
        file.read_to_end(&mut self.data).unwrap();
    }
    pub fn len(&self) -> usize{
        self.data.len()
    }
    pub fn to_vec(&self) -> Vec<u8> {
        self.data.to_vec()
    }
    pub fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }
}

impl Index<usize> for Cartridge {
    type Output = u8;
    fn index(&self, index : usize) -> &u8 {
        &self.data[index]
    }
}

impl IndexMut<usize> for Cartridge{
    fn index_mut(&mut self, index : usize) -> &mut u8 {
        &mut self.data[index]
    }
}


struct CartridgeHeader {
    name : [u8;4],
    prg_rom_size : u8,
    chr_rom_size : u8,
    /// Mapper, mirroring, battery, trainer
    flag6 : u8,
    /// Mapper, VS/Playchoice, NES 2.0
    flag7 : u8,
    /// Program size
    flag8 : u8,
    /// TV system (rarely used extension)
    flag9 : u8,
    /// TV system, PRG-RAM presence (unofficial, rarely used extension)
    flag10 : u8,
    /// Unused padding (should be filled with zero, but some rippers put their name across bytes 7-15)
    padding : [u8;8]
    
}

impl CartridgeHeader {
    fn new() -> CartridgeHeader{
        CartridgeHeader {
            name : [0x4E,0x45,0x53,0x1A],
            prg_rom_size : 0,
            chr_rom_size : 0,
            flag6 : 0,
            flag7 : 0,
            flag8 : 0,
            flag9 : 0,
            flag10 : 0,
            padding : [0;8]
        }
    }
}


//TODO: Check the NES2.0 format for a disambiguation (cf : http://wiki.nesdev.com/w/index.php/INES)

bitflags! {
    pub struct FLAG6 : u8 {
        /// Mirroring : 0 -> horizontal, 1 -> vertical
        const MIRRORING = 1;
        /// Battery-backed ram
        const BAT_RAM = 1<<1;
        /// 512 bytes trainer at 0x7000 to 0x71FF
        const TRAINER = 1<<2;
        /// Ignore mirroring control or above mirroring bit;
        /// instead provide four-screen VRAM
        const IGNORE_MIRROR = 1<<3;
        const LOWER_VAL = 0xF0;

    }
}
bitflags! {
    pub struct FLAG7 : u8 {
        /// VS Unisystem
        const VS = 1;
        /// PlayChoice-10 (8KB of Hint Screen data stored after CHR data)
        const PLAYCHOICE = 1<<1;
        /// If equal to 2, flags 8-15 are in NES 2.0 format
        const TRAINER = 3<<2;
        /// Upper nybble of mapper number
        const LOWER_VAL = 0xF0;

    }
}
bitflags! {
    pub struct FLAG9 : u8 {
        /// TV system (0: NTSC; 1: PAL)
        const TV = 1;
        /// Reserved, set to zero
        const PLAYCHOICE = 0xFE;
    }
}

bitflags! {
    pub struct FLAG10 : u8 {
        /// TV system (0: NTSC; 2: PAL; 1/3: dual compatible)
        const VS = 3;
        /// PRG RAM ($6000-$7FFF) (0: present; 1: not present)
        const PRG_RAM = 1<<4;
        /// 0: Board has no bus conflicts; 1: Board has bus conflicts
        const TRAINER = 1<<5;

    }
}