mod components;
mod utils;

use components::*;
use utils::*;

fn main() {
    let mut bus = Bus::new();
    let cpu = OLC6502::new();
    cpu.write(&mut bus, 0, 1u8);
    println!("{:?}", bus.read(0, true));
}
