use console::Term;

use rand::prelude::*;

use super::{
    OLC6502,
    Bus
};

pub fn test_cpu(cpu : &OLC6502, bus : &Bus ){

    println!("{}\n{}",display_ram(bus.ram.to_vec(), 16, 3),display_registers(cpu));
    
}   

/// Display ram on a length * depth
fn display_ram(ram : Vec<u8>, length : usize, depth : usize) -> String {
    let mut result = String::new();

    for i in 0..depth*length{
        if i % length == 0{
            result+= format!("\n{1:00$X}  --  ",4,i).as_str();
        }
        result+= format!(" {1:00$X}",2,ram[i]).as_str();
    }
    result
    
}

fn display_registers(cpu : &OLC6502) -> String{
    let mut result = String::new();
    result += format!("{:012} : {:0}\n","Accumulator",cpu.a).as_str();
    result += format!("{:012} : {:0}\n","X-Register",cpu.x).as_str();
    result += format!("{:012} : {:0}\n","Y-Register",cpu.y).as_str();
    result += format!("{:012} : {:0}\n","StackPointer",cpu.stkp).as_str();
    result += format!("{:012} : {:0}\n","Status",cpu.status).as_str();
    result
}