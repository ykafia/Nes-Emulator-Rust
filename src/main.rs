mod components;
mod test;
mod utils;
pub use components::*;
pub use test::*;
pub use utils::*;

fn main() {
    let mut nes = NesData::new();
    let mut cpu = CPU6502::new();

    test_cpu(&mut cpu, &mut nes, None);
}
