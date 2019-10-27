use console::Term;
use std::io;
use asm6502::assemble;
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
    cpu.reset(bus);
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
    result += format!("{:012} : {:0}\n","Accumulator",cpu.a).as_str();
    result += format!("{:012} : {:0}\n","X-Register",cpu.x).as_str();
    result += format!("{:012} : {:0}\n","Y-Register",cpu.y).as_str();
    result += format!("{:012} : {:0}\n","StackPointer",cpu.stkp).as_str();
    result += format!("{:012} : {:0}\n","Status",cpu.status).as_str();
    result += format!("{:012} : {:0}\n","PC",cpu.pc).as_str();
    result
}


/// This functions returns a compiled assembly code  that 
/// loads some data in the ram and executes some shift left
/// Here is the assembly source
/// LDA #$05
/// STA $0200
/// ASL A
/// STA $0201
fn test_code() -> Vec<u8>{
    // vec!(8)
    let mut result : Vec<u8> = Vec::new();
    let mut file = File::open("src/test/main.asm").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    assemble(contents.as_bytes(), &mut result).unwrap();
    result
}