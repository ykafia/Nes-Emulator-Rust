pub struct APU {
    pub registers : [u8; 8],
}

impl APU {
    pub fn new() -> APU {
        APU {
            registers : [0;8] 
        }
    }
}