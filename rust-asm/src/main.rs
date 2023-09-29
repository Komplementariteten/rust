#![feature(asm_const)]
use std::arch::asm;

fn main() {

    let i: u64 = 3;
    let o: u64;
    unsafe {
        asm!(
        "mov {0}, {1}",
        "add {0}, {number}",
            out(reg) o,
            in(reg) i,
            number = const 5,
        );
    }
    assert_eq!(o, 8);
    println!("Hello, world!");
}
