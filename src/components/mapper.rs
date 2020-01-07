use super::*;

pub struct Mapper;

impl Mapper {

    fn map(id : u8, src: Source, addr: u16, n_p_banks : usize) -> (bool,usize){
        match id {
            0 => Mapper::mapper_000(src, addr, n_p_banks),
            _ => (false,0),
        }
    }

    fn mapper_000(src: Source, addr: u16, n_p_banks : usize) -> (bool,usize) {
        match src {
            Source::CPU => {
                if addr >= 0x8000{
                    if n_p_banks > 1 {(true,0x7FFF)} else {(true,0x3FFF)}
                }
                else {
                    (false,0)
                }
            },
            Source::PPU => {
                if addr <= 0x0FFF {
                    (true,addr.into())
                }
                else {
                    (false,0)
                }
            },
        }
    }
}