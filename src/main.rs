mod byte_types;
use byte_types::*;
mod cpu_type;
use cpu_type::*;
mod common_traits;
use common_traits::*;

fn main() {
    let mut x = BaseByte::new();
    x.set_c(0b101);
    println!("{}", x.to_byte());
}
