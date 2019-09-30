mod components;
mod utils;

use components::*;
use utils::*;

fn main() {
    let mut x = BaseByte::new();
    x.set_c(0b101);
    println!("{}", x.to_byte());
}
