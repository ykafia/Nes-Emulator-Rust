use std::ops::{Index,IndexMut};
use std::fs::File;
use std::io::prelude::*;
use super::*;
#[warn(non_camel_case_types)] 
pub struct Cartridge {
    /// The complete ROM
    pub data : Vec<u8>,
    /// ROM containing the program
    pub program_rom : Vec<u8>,
    /// ROM containing the sprites
    pub character_rom : Vec<u8>,
    /// Id of the mapper
    pub mapper_id : u8,
    /// Number of program banks
    pub prgm_bank_n : u8,
    /// Number of CHR banks
    pub chr_bank_n : u8,

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
        match Self::get_type(self.data.to_vec()) {
            NesFileType::INES => {
                let x = INes::new(self.data.to_vec());
                self.program_rom = self.data[15..x.get_prg_size()].to_vec();
                self.character_rom = self.data[x.get_prg_size()..x.get_chr_size()].to_vec();
                self.mapper_id = x.get_mapper_id();
                
            },
            // TODO : read NES2
            NesFileType::NES2 => {
                let x = Nes2::new(self.data.to_vec());
                self.program_rom = self.data[15..x.get_prg_size()].to_vec();
                self.character_rom = self.data[x.get_prg_size()..x.get_chr_size()].to_vec();
                self.mapper_id = x.get_mapper_id();
            }
        }
        
        
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
    pub fn get_type(data : Vec<u8>) -> NesFileType{
        if data[7] & 0x0C == 0 && data[12..15].to_vec().iter().sum::<u8>() == 0 {
            NesFileType::INES
        } else /*if self.data[7] & 0x0C == 0x08 */{
            NesFileType::NES2
        }
    }
    pub fn ppu_read(&self, addr : u16) -> (bool,u8) {
        (true,0)
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

impl ReadWriteFunc for Cartridge {
    fn cpu_read(nes: &mut NesData, addr: u16, read_only: bool) -> u8 {        
        nes.read(addr, read_only, None)
    }
    fn cpu_write(nes: &mut NesData, addr: u16, data: u8) {
        nes.write(addr, data, None);
    }
    fn ppu_read(ppu: &mut PPU, addr: u16, read_only: bool) -> u8 {
        ppu.read(addr,read_only)
    }
    fn ppu_write(ppu: &mut PPU, addr: u16, data: u8){
        ppu.write(addr,data);
    }
}

// region Nes files
pub struct INes {
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
}

pub struct Nes2 {
    /// [0-3]  Name. 
    name : [u8;4],
    /// [4] Program rom size 
    prg_rom_size : u8,
    /// [5] CHR rom size 
    chr_rom_size : u8,
    /// [6] Mapper, mirroring, battery, trainer
    flag6 : u8,
    /// [7] Mapper, VS/Playchoice, NES 2.0
    flag7 : u8,
    /// [8] Mapper value for NES2.0 files, byte 8
    mapper : u8,
    /// [9] PRG + CHR rom size contained in one byte, PRG -> low, CHR -> high
    rm_sizes_msb : u8,
    /// [10] PRG-RAM/EEPROM size.
    /// low -> PRG-RAM (volatile) shift count.
    /// high -> PRG-NVRAM/EEPROM (non-volatile) shift count.
    /// If the shift count is zero, there is no PRG-(NV)RAM.
    /// If the shift count is non-zero, the actual size is
    /// "64 << shift count" bytes, i.e. 8192 bytes for a shift count of 7. 
    eeprom_size : u8,
    /// [11] low -> CHR-RAM size (volatile) shift count
    /// high -> CHR-NVRAM size (non-volatile) shift count
    /// If the shift count is zero, there is no CHR-(NV)RAM.
    /// If the shift count is non-zero, the actual size is
    /// "64 << shift count" bytes, i.e. 8192 bytes for a shift count of 7.
    chr_ram_size : u8,
    /// [12] CPU/PPU Timing 
    /// only 4 different values (only 2 lowest bits used)
    /// 0: RP2C02 ("NTSC NES")
    /// 1: RP2C07 ("Licensed PAL NES")
    /// 2: Multiple-region
    /// 3: UMC 6527P ("Dendy")
    cpu_ppu_timing : u8,
    /// [13] Vs. System Type (when data[7] & 3 = 3)
    /// low : PPU Type
    /// high : hardware type
    vs_system_type : u8,
    /// [13] Extended Console Type (when data[7] & 3 =3)
    extended_console_type : u8,
    /// [14] Miscellaneous ROMs : Number of miscellaneous ROMs present (max = 3)
    misc_roms : u8,
    /// [15] Default Expansion Device
    default_expansion_dvc : u8

}

impl Nes2 {
    fn new(data : Vec<u8>) -> Nes2 {
        Nes2 {
            name : [ 
                data[0],
                data[1],
                data[2],
                data[3]
            ],
            prg_rom_size : data[4],
            chr_rom_size : data[5],
            flag6 : data[6],
            flag7 : data[7],
            mapper : data[8],
            rm_sizes_msb : data[9],
            eeprom_size : data[10],
            chr_ram_size : data[11],
            cpu_ppu_timing : data[12],
            vs_system_type : match data[7] & 3 == 1 {
                true => data[13],
                false => 0
            },
            extended_console_type : match data[7] & 3 == 3 {
                true => data[13],
                false => 0
            },
            misc_roms : data[14],
            default_expansion_dvc : data[15]
            
        }
    }
}

impl INes {
    fn new(data : Vec<u8>) -> INes {
        INes {
            name : [ 
                data[0],
                data[1],
                data[2],
                data[3]
            ],
            prg_rom_size : data[4],
            chr_rom_size : data[5],
            /// Mapper, mirroring, battery, trainer
            flag6 : data[6],
            /// Mapper, VS/Playchoice, NES 2.0
            flag7 : data[7],
            /// Program size
            flag8 : data[8],
            /// TV system (rarely used extension)
            flag9 : data[9],
            /// TV system, PRG-RAM presence (unofficial, rarely used extension)
            flag10 : data[10],
        }
    }
    
}

pub enum NesFileType{
    INES,
    NES2
}




pub trait HeaderData {
    fn get_prg_size(&self) -> usize;
    fn get_chr_size(&self) -> usize;
    fn get_name(&self) -> Vec<u8>;
    fn get_flag6(&self) -> u8;
    fn get_flag7(&self) -> u8;
    fn get_mapper_id(&self) -> u8;
}


impl HeaderData for INes{
    fn get_prg_size(&self) -> usize{
        16384 * self.prg_rom_size as usize
    }
    fn get_chr_size(&self) -> usize{
        8192 * self.chr_rom_size as usize  
    }
    fn get_name(&self) -> Vec<u8>{
        self.name.to_vec()
    }
    fn get_flag6(&self) -> u8{
        self.flag6
    }
    fn get_flag7(&self) -> u8 {
        self.flag7
    }
    fn get_mapper_id(&self) -> u8 {
        0
    }
}
impl HeaderData for Nes2{
    fn get_prg_size(&self) -> usize{
        16384 * self.prg_rom_size as usize
    }
    fn get_chr_size(&self) -> usize{
        8192 * self.chr_rom_size as usize  
    }
    fn get_name(&self) -> Vec<u8>{
        self.name.to_vec()
    }
    fn get_flag6(&self) -> u8{
        self.flag6
    }
    fn get_flag7(&self) -> u8 {
        self.flag7
    }
    fn get_mapper_id(&self) -> u8 {
        self.mapper
    }
}
// endregion
//TODO: Check the NES2.0 format for a disambiguation (cf : http://wiki.nesdev.com/w/index.php/INES)


// region BITFLAGS
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

// endregion