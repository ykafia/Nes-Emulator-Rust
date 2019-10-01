use super::cpu::*;
/// This function generates a 16x16 vector for the lookup table of op codes
pub fn get_lookup_list() -> Vec<Vec<INSTRUCTION>>{
    vec![
        //line 1
        vec![
            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMP".to_string(), cycles : 7},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IDX".to_string(), cycles : 6},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"ZP0".to_string(), cycles : 3},
            INSTRUCTION {opcode:"ASL".to_string(),addr_mode:"ZP0".to_string(), cycles : 5},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"PHP".to_string(),addr_mode:"IMP".to_string(), cycles : 3},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 2},
            INSTRUCTION {opcode:"ASL".to_string(),addr_mode:"IMP".to_string(), cycles : 2},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"ABS".to_string(), cycles : 4},
            INSTRUCTION {opcode:"ASL".to_string(),addr_mode:"ABS".to_string(), cycles : 6},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
        ],
        // line 2
        vec![
            INSTRUCTION {opcode:"BPL".to_string(),addr_mode:"REL".to_string(), cycles : 2},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IDY".to_string(), cycles : 5},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"ZPX".to_string(), cycles : 4},
            INSTRUCTION {opcode:"ASL".to_string(),addr_mode:"ZPX".to_string(), cycles : 6},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"CLC".to_string(),addr_mode:"IMP".to_string(), cycles : 2},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"ABY".to_string(), cycles : 4},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"ABX".to_string(), cycles : 4},
            INSTRUCTION {opcode:"ASL".to_string(),addr_mode:"ABX".to_string(), cycles : 7},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
        ],
        //line 3
        vec![
            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
        ],
        //line 4
        vec![
            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
        ],
        //line 5
        vec![
            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
        ],
        //line 6
        vec![
            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
        ],
        //line 7
        vec![
            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
        ],
        //line 8
        vec![
            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
        ],
        // line 9
        vec![
            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
        ],
        // line 10
        vec![
            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
        ],
        // line 11
        vec![
            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
        ],
        // line 12
        vec![
            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
        ],
        // line 13
        vec![
            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
        ],
        // line 14
        vec![
            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
        ],
        // line 15
        vec![
            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
        ],
        // line 16
        vec![
            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},

            INSTRUCTION {opcode:"BRK".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"ORA".to_string(),addr_mode:"IMM".to_string(), cycles : 1},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
            INSTRUCTION {opcode:"???".to_string(),addr_mode:"???".to_string(), cycles : 0},
        ]
        
}