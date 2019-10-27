use console::Term;
use std::io;

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
    
    // Convert string to bytes
    let code : Vec<u8> =  test_code().split_whitespace()
                                     .map(|x| u8::from_str_radix(&x,16).unwrap())
                                     .collect();
    // Writes the code in the ram with offset 0x8000
    for i in 0..code.len(){
        bus.ram[0x8000+i] = code[i];
    }
    cpu.reset(bus);
    while input.trim() != "quit" {

        cpu.clock(bus);
        println!("{}\n{}\n\n\n\nCode :\n\n{}",  display_registers(cpu),
                        display_ram(bus.ram.to_vec() , 16, dpth),
                        display_code(bus.ram.to_vec(), 16, dpth));
        io::stdin().read_line(&mut input).unwrap();
        term.clear_screen().unwrap();
    }
    

    
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
/// ASL A
/// STA $0200
fn test_code() -> String{
    
    "A9 05 0A 8D 00 02".to_string()

}