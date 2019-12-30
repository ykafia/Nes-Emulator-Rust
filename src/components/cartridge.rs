use std::ops::{Index,IndexMut};
use std::fs::File;
use std::io::prelude::*;

pub struct Cartridge {
    pub data : [u8; 0xBFDF]
}

impl Cartridge {
    pub fn new() -> Cartridge {
        Cartridge {
            data : [0u8; 0xBFDF]
        }
    }
    pub fn load(&mut self, pathfile:&str) {
        let mut file = File::open(pathfile).unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        match contents.len() > self.len() {
            true => {
                for i in 0..contents.len() {
                    self.data[i] = contents[i];
                }
            },
            false => panic!("The file is too large to be loaded by the NES.")
        }
    }
    pub fn len(&self) -> usize{
        self.data.len()
    }
    pub fn to_vec(&self) -> Vec<u8> {
        self.data.to_vec()
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