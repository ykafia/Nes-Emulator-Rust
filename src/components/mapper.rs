use super::*;

pub struct Mapper {
    pub mapper_id: u16,
    pub nb_prg_banks: u16,
    pub nb_chr_banks: u16,
}

pub enum Source {
    CPU,
    PPU,
}

impl Mapper {
    pub fn new(header: Vec<u8>) -> Self {
        Mapper {
            mapper_id: 0,
            nb_prg_banks: 0,
            nb_chr_banks: 0,
        }
    }

    /// General mapper function
    pub fn map(&self, src: Source, addr: u16) -> Option<usize> {
        match self.mapper_id {
            0 => self.mapper_000(src, addr),
            _ => None,
        }
    }

    fn mapper_000(&self, src: Source, addr: u16) -> Option<usize> {
        match src {
            Source::CPU => {
                if addr >= 0x8000 {
                    if self.nb_prg_banks > 1 {
                        Some(addr as usize & 0x7FFF)
                    } else {
                        Some(addr as usize & 0x3FFF)
                    }
                } else {
                    None
                }
            }
            Source::PPU => {
                if addr <= 0x0FFF {
                    Some(addr.into())
                } else {
                    None
                }
            }
        }
    }
}
