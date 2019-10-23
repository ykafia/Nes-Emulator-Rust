mod components;
mod utils;
mod test;
use components::*;
use test::*;
// use utils::*;

fn main() {
    let mut bus = Bus::new();
    let mut cpu = OLC6502::new();
    cpu.write(&mut bus, 0, 1u8);

    test_cpu(&cpu, &bus, None);
}
