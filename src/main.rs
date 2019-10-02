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
    println!("there are {} set of {} instructions",y.len(),y[0].len());
    println!("{:#?}",y[0][2].opcode);
}
