mod components;
mod test;
mod utils;
use components::*;
use test::*;

fn main() {
    let mut nes = NesData::new();
    let mut cpu = CPU6502::new();

    test_cpu(&mut cpu, &mut nes, None);
}
