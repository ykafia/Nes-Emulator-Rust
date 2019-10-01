use super::cpu::*;
use std::fs;
use std::str::FromStr;


/// This function generates a 16x16 vector for the lookup table of op codes
pub fn get_lookup_list() -> Vec<Vec<INSTRUCTION>>{
    let data = fs::read_to_string("src/instructions.txt").expect("unable to read file");
    let data_lines = data.lines();
    
    let mut result : Vec<Vec<INSTRUCTION>> = Vec::new();
    let mut tmp : Vec<INSTRUCTION> = Vec::new();
    
    for line in data_lines{
        match line{
            "???" => tmp.push(
                INSTRUCTION {
                    opcode : "???".to_string(),
                    addr_mode : "???".to_string(),
                    cycles : 0
                }
            ),
            "#line" => {
                result.push(tmp);
                tmp = Vec::new();
                },
            x => {
                let y = x.to_string();
                let ins_data : Vec<&str> = y.split_whitespace().collect();
                tmp.push(
                    INSTRUCTION {
                        opcode : ins_data[0].to_string(),
                        addr_mode : ins_data[1].to_string(),
                        cycles : u8::from_str(ins_data[2]).expect("Failed parsing the number")
                    }
                )
            }
        }
    }

    result
}