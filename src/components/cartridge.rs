use std::{convert::TryInto, ops::{Index, IndexMut}};


pub struct Cartridge{
    // Actual rom containing all the data
    rom : [u8; 0xBFDF],
    pub header : Header, 
}

#[derive(Default)]
pub struct Header {
    /// "NES" followed by eol
    pub nes_validator : [u8;4],
    /// size of the program rom
    pub size_prg : u8,
    /// size of the character rom
    pub size_chr : u8,
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
            rom : [0; 0xBFDF],
            header: Header::default(),
        }
    }
    pub fn load(&mut self, data : [u8; 0xBFDF]){
        self.rom = data;
        self.header = Header::new(data[0..15].try_into().expect("Bad data input"));
    }
    pub fn to_vec(&mut self) -> Vec<u8>{
        self.rom.to_vec()
    }
}


impl Header {
    pub fn new(data : [u8; 16]) -> Self {
        Header {
            nes_validator: data[0..3].try_into().expect("Wrong input data"),
            size_prg: data[4],
            size_chr: data[5],
            flags_6: data[6],
            flags_7: data[7],
            flags_8: data[8],
            flags_9: data[9],
            flags_10: data[10],
            padding: data[11..15].try_into().expect("wrong data input"),
        }
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

