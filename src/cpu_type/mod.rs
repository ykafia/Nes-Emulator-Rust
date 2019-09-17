#![allow(non_snake_case)]
use super::common_traits::*;

trait CPUFunctions{
    
    fn get_flag(self,f:FLAGS6502)->u8;
    fn set_flag(self,f:FLAGS6502,v:bool)->();
    
    fn IMP() ->u8;    
    fn IMM()->u8;
    fn ZP0()->u8;    
    fn ZPX()->u8;
    fn ZPY()->u8;    
    fn REL()->u8;
    fn ABS()->u8;    
    fn ABX()->u8;
    fn ABY()->u8;
    fn IND()->u8;
    fn IZX()->u8;
    fn IZY()->u8;

    //TODO: Add the other CPU instructions [http://obelisk.me.uk/6502/reference.html]
    fn ADC()->u8; fn AND()->u8; fn ASL()->u8; fn BCC()->u8;
    fn BCS()->u8; fn BEQ()->u8; fn BIT()->u8; fn BMI()->u8;
    fn BNE()->u8; fn BPL()->u8; fn BRK()->u8; fn BVC()->u8;
    fn BVS()->u8; fn CLC()->u8; fn CLD()->u8; fn CLI()->u8;
    fn CLV()->u8; fn CMP()->u8; fn CPX()->u8; fn CPY()->u8;
    fn DEC()->u8; fn DEX()->u8; fn DEY()->u8; fn EOR()->u8;
    fn INC()->u8; fn INX()->u8; fn INY()->u8; fn JMP()->u8;
    fn JSR()->u8; fn LDA()->u8; fn LDX()->u8; fn NOP()->u8;
    fn ORA()->u8; fn PHA()->u8; fn PHP()->u8; fn PLA()->u8;
    fn PLP()->u8; fn ROL()->u8; fn ROR()->u8; fn RTI()->u8;
    fn RTS()->u8; fn SBC()->u8; fn SEC()->u8; fn SED()->u8;
    fn SEI()->u8; fn STA()->u8; fn STX()->u8; fn STY()->u8;
    fn TAX()->u8; fn TAY()->u8; fn TSX()->u8; fn TXA()->u8;
    fn TXS()->u8; fn TYA()->u8;

    fn OpCodes(); // Unintended operations
    // Those are the clock based functions
    fn clock(); // This should control the number of clock cycles each instructions takes.
    fn reset();
    fn interupt_req();
    fn non_maskable_interupt_req();
    fn fetch_data(); // it should fetch data


}

struct CPU {
    a: u8,
    x: u8,
    y: u8,
    stkp: u8,
    pc: u16,
    status: u8,
    data:u8, // Data that can be fetched for some operations when needed
    addr_abs:u16, // Absolute Adress to another data source needed
    addr_rel:u16,
    curr_opcode:u8, // Opcode currently running 
    cycles:u8 // number of cycles left for the current opcode to finish

}
enum FLAGS6502 {
    C = 1 << 0, // Carry bit
    Z = 1 << 1, // Zero
    I = 1 << 2, // Disable interrupts
    D = 1 << 3, // Decimal mode
    B = 1 << 4, // Break
    U = 1 << 5, // Unused
    V = 1 << 6, // Overfow
    N = 1 << 7, // Negative
}


trait InstructionFunctions{
    fn operate(&self);
    fn addrmode(&self);
}

struct INSTRUCTION{
    name:String,
    cycles:u8
}
//TODO: Implement InstructionFunctions
// This trait should pattern match the name and call a specific instruction
// Should work as a pointer of function
