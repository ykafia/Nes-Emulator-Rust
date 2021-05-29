use std::{convert::TryInto, fmt, ops::{Index, IndexMut}};

use super::{Mapper, Source};


pub struct Cartridge{
    // Actual rom containing all the data
    rom : Vec<u8>,
    pub header : Header,
    pub prg_memory : Vec<u8>,
    pub chr_memory : Vec<u8>,
    pub mapper : Mapper
}

#[derive(Default,Debug)]
pub struct Header {
    /// "NES" followed by eol
    pub nes_validator : [u8;4],
    /// size of the program rom
    pub nb_prg_banks : u8,
    /// size of the character rom
    pub nb_chr_banks : u8,
    /// Mapper, mirroring, battery, trainer
    pub flags_6 : u8,
    /// Mapper, VS/Playchoice, Nes2.0
    pub flags_7 : u8,
    /// PRG RAM size, rarely used
    pub flags_8 : u8,
    /// TV System, rarely used
    pub flags_9 : u8,
    /// TV System, PRG RAM presence, rarely used
    pub flags_10 : u8,
    /// Empty padding, used to store names of hackers
    pub padding : [u8;5],
}

impl Cartridge {
    pub fn new() -> Self {
        Cartridge {
            rom : Vec::new(),
            header: Header::default(),
            mapper: Mapper::default(),
            prg_memory : Vec::new(),
            chr_memory : Vec::new()
        }
    }
    pub fn load(&mut self, data : Vec<u8>){
        self.rom = data;
        self.header = Header::new(&self.rom[0..16]);
        let mut offset = 16;
        if self.header.has_trainer() {offset += 512};
        let nb_bank = self.header.nb_prg_banks as usize;
        let nb_bank2 = self.header.nb_chr_banks as usize;
        self.prg_memory = self.rom[
                offset .. offset + self.header.nb_prg_banks as usize * 0x4000
        ].to_vec();
        offset +=self.header.nb_prg_banks as usize * 0x4000;
        if self.header.nb_chr_banks > 0 {
            self.chr_memory = self.rom[
                offset .. offset + self.header.nb_chr_banks as usize * 0x2000
            ].to_vec();
        }
                

    }
    pub fn to_vec(&self) -> Vec<u8>{
        self.rom.to_vec()
    }

    pub fn ppu_read(&self, addr : u16) -> u8 {
        match self.mapper.map(Source::PPU,addr) {
            Some(a) => self.chr_memory[a],
            None => 0
        }
        
    }
    pub fn cpu_read(&self, addr : u16) -> u8 {
        match self.mapper.map(Source::CPU,addr) {
            Some(a) => self.prg_memory[a],
            None => 0
        }
    }
    pub fn ppu_write(&mut self, addr : u16, data : u8) {
        match self.mapper.map(Source::PPU,addr) {
            Some(a) => self.chr_memory[a] = data,
            None => ()
        }
        
    }
    pub fn cpu_write(&mut self, addr : u16, data : u8) {
        match self.mapper.map(Source::CPU,addr) {
            Some(a) => self.prg_memory[a] = data,
            None => ()
        }
    }
}


impl Header {
    pub fn new(data : &[u8]) -> Self {
        Header {
            nes_validator: [data[0],data[1],data[2],data[3]],
            nb_prg_banks: data[4],
            nb_chr_banks: data[5],
            flags_6: data[6],
            flags_7: data[7],
            flags_8: data[8],
            flags_9: data[9],
            flags_10: data[10],
            padding: [data[11],data[12],data[13],data[14],data[15]],
        }
    }
    pub fn mapper_id(&self) -> u16 {
        ((self.flags_7 >> 4) << 4 | self.flags_6 >> 4) as u16
    }
    pub fn is_nes2(&self) -> bool {
        self.flags_7 & 0b00001100 == 0x8
    }
    pub fn has_trainer(&self) -> bool{
        self.flags_6 & 0x4 == 1
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = self.nes_validator.to_vec();
        let validation = std::str::from_utf8(v.as_slice()).expect("Not a nes file");
        
        
        write!(f, 
            "
                File validation : {}\n
                is nes 2.0 : {}\n
                Size prg : {}\n
                Size chr : {}\n
                mapper id : {}\n
            ", 
            validation, 
            self.is_nes2(),
            self.nb_prg_banks,
            self.nb_chr_banks,
            self.mapper_id()
        )
    }
}


impl Index<usize> for Cartridge {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.rom[index]
    }
}
impl IndexMut<usize> for Cartridge {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rom[index]
    }
}

