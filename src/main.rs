mod components;
mod utils;

use components::*;
use utils::*;

fn main() {
    let mut bus = Bus::new();
    let mut cpu = OLC6502::new();
    cpu.write(&mut bus, 0, 1u8);
    println!("{:?}", bus.read(0, true));
    let y = get_lookup_list();
    println!("there are {} one of them is a {} instruction", y.len(), y[0].opcode);
}
