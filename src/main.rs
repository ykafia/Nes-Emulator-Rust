mod components;
mod utils;
mod test;
use components::*;
use test::*;
// use utils::*;

fn main() {
    let mut bus = Bus::new();
    let mut cpu = OLC6502::new();

    test_cpu(&mut cpu, &mut bus, None);
}
