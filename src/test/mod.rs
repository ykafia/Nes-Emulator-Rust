use console::Term;
use std::io;
use std::fs::File;
use std::io::prelude::*;

use super::*;

pub fn test_cpu(cpu : &mut OLC6502, bus : &mut Bus, depth : Option<usize> ){
    let dpth = match depth{
        Some(x) => x,
        None => 8
    };
    let mut input = String::new();


    
    let term = Term::stdout();
    // Set the reset vector
    bus.ram[0xFFFC] = 0x00;
    bus.ram[0xFFFD] = 0x80;
    
    // get the Assembly code
    let code : Vec<u8> =  test_code();
    // Writes the code in the ram with offset 0x8000
    for i in 0..code.len(){
        bus.ram[0x8000+i] = code[i];
    }
    cpu.power(bus);
    while input.trim() != "quit" {

        cpu.clock(bus);
        println!("{}\n{}\n\n\n\nCode :\n\n{}",  display_registers(cpu),
                        display_ram(bus.ram.to_vec(), 0x0200 , 16, dpth),
                        display_code(bus.ram.to_vec(), 16, dpth));
        io::stdin().read_line(&mut input).unwrap();
        term.clear_screen().unwrap();
    }
    

    
}   

/// Display ram on a length * depth
fn display_ram(ram : Vec<u8>, start : usize, length : usize, depth : usize) -> String {
    let mut result = String::new();

    for i in start..start + depth*length{
        if i % length == 0{
            result+= format!("\n{1:00$X}  --  ",4,i).as_str();
        }
        result+= format!(" {1:00$X}",2,ram[i]).as_str();
    }
    result
    
}

fn display_code(ram : Vec<u8>, length : usize, depth : usize) -> String{
    let mut result = String::new();

    for i in 0x8000..0x8000+depth*length{
        if i % length == 0{
            result+= format!("\n{1:00$X}  --  ",4,i).as_str();
        }
        result+= format!(" {1:00$X}",2,ram[i]).as_str();
    }
    result
}

fn display_registers(cpu : &OLC6502) -> String{
    let mut result = String::new();
    result += format!("{:012} : {1:02$X}\n","Accumulator",cpu.a,2).as_str();
    result += format!("{:012} : {1:02$X}\n","X-Register",cpu.x,2).as_str();
    result += format!("{:012} : {1:02$X}\n","Y-Register",cpu.y,2).as_str();
    result += format!("{:012} : {1:02$X}\n","StackPointer",cpu.stkp,2).as_str();
    result += format!("{:012} : {1:02$X}\n","Status",cpu.status,2).as_str();
    result += format!("{:012} : {1:02$X}\n","PC",cpu.pc,2).as_str();
    result
}


/// This functions returns a compiled assembly code  that 
/// loads some data in the ram and executes some shift left
fn test_code() -> Vec<u8>{

    let mut file = File::open("src/test/test.nes").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    contents
    
}