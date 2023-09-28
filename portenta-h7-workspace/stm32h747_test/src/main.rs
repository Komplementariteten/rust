#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln as log, rtt_init_print as log_init};
use stm32h747::Peripherals;

#[entry]
fn main() -> ! {
    log_init!();
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();
    log!("ready");
    // while dp.RCC.cr.read().pll1rdy().bit() {}

    // delay config
    // enable channel 1
    let dac1 = dp.DAC.cr.write(| w | {
        w.en1().set_bit()
    });

    dp.DAC.cr.write(| w| unsafe {
        w.bits(0b1111110000011)
    });

    // Power config
    let status = dp.DAC.sr.read().bits();
    log!("{:b}", status);

    let status = dp.DAC.cr.read().bits();
    log!("{:b}", status);

    loop {
        /* let v = dp.DAC.dhr8r1.write(| w | unsafe {
            w.bits(2096)
        });
        let status = dp.DAC.sr.read().bits();
        log!("{:b}", status); */
    }
}
