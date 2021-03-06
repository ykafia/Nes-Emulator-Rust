use console::Term;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use super::*;


pub fn test_cpu(cpu: &mut CPU6502, nes: &mut NesData, depth: Option<usize>) {
    let dpth = match depth {
        Some(x) => x,
        None => 8,
    };
    let mut input = String::new();

    let term = Term::stdout();
    // Set the reset vector to the cartridge memory address
    nes.cpu_write(0xFFFC, 0x20);
    nes.cpu_write(0xFFFD, 0x40);

    // get the Assembly code
    let code: Vec<u8> = test_code();
    // Writes the code in the ram with offset 0x8000
    for i in 0..code.len() {
        nes.cpu_write(0x4020 + i as u16, code[i]);
    }
    cpu.power(nes);
    while input.trim() != "quit" {
        cpu.clock(nes);
        println!(
            "{}\n{}\n\n\n\nCode :\n\n{}",
            display_registers(cpu),
            display_ram(nes.ram.to_vec(), 0x0000, 16, dpth),
            display_code(nes.cartridge.to_vec(), 16, dpth)
        );
        io::stdin().read_line(&mut input).unwrap();
        term.clear_screen().unwrap();
    }
}

/// Display ram on a length * depth
fn display_ram(ram: Vec<u8>, start: usize, length: usize, depth: usize) -> String {
    let mut result = String::new();

    for i in start..start + depth * length {
        if i % length == 0 {
            result += format!("\n{1:00$X}  --  ", 4, i).as_str();
        }
        result += format!(" {1:00$X}", 2, ram[i]).as_str();
    }
    result
}

fn display_code(cartridge: Vec<u8>, length: usize, depth: usize) -> String {
    let mut result = String::new();

    for i in 0..depth * length {
        if i % length == 0 {
            result += format!("\n{1:00$X}  --  ", 4, i+0x4020).as_str();
        }
        result += format!(" {1:00$X}", 2, cartridge[i]).as_str();
    }
    result
}

fn display_registers(cpu: &CPU6502) -> String {
    let mut result = String::new();

    result += format!(
        "{:012} : {:03}\n",
        "Applying", cpu.lookup[cpu.curr_opcode as usize].opcode
    )
    .as_str();
    result += format!(
        "{:012} : {:03}\n\n",
        "Adressing", cpu.lookup[cpu.curr_opcode as usize].addr_mode
    )
    .as_str();
    result += format!("{:012} : {1:02$X}\n", "Accumulator", cpu.a, 2).as_str();
    result += format!("{:012} : {1:02$X}\n", "X-Register", cpu.x, 2).as_str();
    result += format!("{:012} : {1:02$X}\n", "Y-Register", cpu.y, 2).as_str();
    result += format!("{:012} : {1:02$X}\n", "StackPointer", cpu.stkp, 2).as_str();
    result += format!("{:012} : {1:02$X}\n", "Status", cpu.status, 2).as_str();
    result += format!("{:012} : {1:02$X}\n", "PC", cpu.pc, 2).as_str();
    result += format!("{:012} : {1:02}\n", "Cycles left", cpu.cycles).as_str();
    result
}

/// This functions returns a compiled assembly code  that
/// loads some data in the ram and executes some shift left
fn test_code() -> Vec<u8> {
    let mut file = File::open("src/test/test.nes").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    contents
}

fn load_mario() -> Vec<u8> {
    let mut file = File::open("roms/Super Mario Bros.nes").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    contents
}
fn load_metroid() -> Vec<u8> {
    let mut file = File::open("roms/Metroid.nes").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    contents
}

#[test]
pub fn metroid_header(){ 
    let mut cartridge = Cartridge::new();
    cartridge.load(load_mario());
    println!("{}",cartridge.header);
}


#[test]
pub fn test_mapper1_read(){
    let mut cartridge = Cartridge::new();
    cartridge.load(load_mario());
    assert_eq!(cartridge.prg_memory[0], cartridge.cpu_read(0x8000));
    assert_eq!(cartridge.prg_memory[0x0032], cartridge.cpu_read(0x8000 + 0x0032));
    assert_eq!(cartridge.chr_memory[0], cartridge.ppu_read(0x0000));
    assert_eq!(cartridge.chr_memory[0x0032], cartridge.ppu_read(0x0032));
}

