#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().expect("Failed to get cortex Peripherals");
}
