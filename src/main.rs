mod components;
mod utils;

use components::{
    CPU6502
};
use utils::{
    BaseByte
};

fn main() {
    let mut x = BaseByte::new();
    x.set_c(0b101);
    //println!("{}", x.to_byte());
}
