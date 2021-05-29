use super::*;
#[derive(Default)]
pub struct Mapper {
    pub mapper_id: u16,
    pub nb_prg_banks: usize,
    pub nb_chr_banks: usize,
}

pub enum Source {
    CPU,
    PPU,
}

impl Mapper {
    pub fn new(header: Header) -> Self {
        Mapper {
            mapper_id: header.mapper_id(),
            nb_prg_banks: header.nb_prg_banks.into(),
            nb_chr_banks: header.nb_chr_banks.into(),
        }
    }

    /// General mapper function
    pub fn map(&self, src: Source, addr: u16) -> Option<usize> {
        match self.mapper_id {
            0 => self.mapper_000(src, addr),
            1 => self.mapper_001(src, addr),
            _ => None,
        }
    }

    fn mapper_000(&self, src: Source, addr: u16) -> Option<usize> {
        match src {
            Source::CPU => {
                if addr >= 0x8000 {
                    if self.nb_prg_banks > 1 {
                        Some((addr as usize - 0x8000) & 0x7FFF)
                    } else {
                        Some((addr as usize - 0x8000) & 0x3FFF)
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
    fn mapper_001(&self, src: Source, addr: u16) -> Option<usize> {
        
    }
}
