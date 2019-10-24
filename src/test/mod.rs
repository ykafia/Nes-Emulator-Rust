use console::Term;

use super::{
    OLC6502,
    Bus
};

pub fn test_cpu(cpu : &mut OLC6502, bus : &mut Bus, depth : Option<usize> ){
    let dpth = match depth{
        Some(x) => x,
        None => 8
    };

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



    println!("{}\n{}\n\n\n\nCode :\n\n{}",  display_registers(cpu),
                        display_ram(bus.ram.to_vec() , 16, dpth),
                        display_code(bus.ram.to_vec(), 16, dpth));
    
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
    result
}


/// This functions returns a compiled assembly code  that 
/// loads some data in the ram and executes some shift left
/// Here is the assembly source
/// LDA #$01
/// ASL A
/// ASL A 
/// ASL A 
/// STA $0200
/// LDA #$05
/// STA $0201
/// LDA #$08
/// STA $0202
fn test_code() -> String{
    
    "A9 01 0A 0A 0A 8D 00 02 A9 05 8D 01 02 A9 08 8D 02 02".to_string()

}