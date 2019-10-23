use console::Term;

use rand::prelude::*;

use super::{
    OLC6502,
    Bus
};

pub fn test_cpu(cpu : OLC6502, bus : Bus ){
    let mut x : Vec<u8> = Vec::new();
    for _ in 0..64{
        x.push(random());
    }
    println!("{}",display_ram(x, 3, 16));
    

}   

/// Display ram on a length * depth
fn display_ram(ram : Vec<u8>, length : usize, depth : usize) -> String {
    let mut result = String::new();
    for i in 64..128{
        result+= format!("{1:00$x}\n",4,i).as_str();
    }
    // for i in 0..depth{
        
    //     result+= format!(" {:x} ",ram[i]).as_str();
        
    // }
    result
    
}