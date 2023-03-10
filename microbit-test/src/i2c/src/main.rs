#![no_main]
#![no_std]

use core::str;

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprint, rprintln, rtt_init_print};

use microbit::{
    hal::twim,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
    pac::twim0::frequency::FREQUENCY_A,
};

use core::fmt::Write;
use heapless::Vec;
use lsm303agr::{AccelOutputDataRate, Lsm303agr, MagOutputDataRate};
use microbit::hal::prelude::*;
use microbit::hal::Timer;
use nb::block;

mod serial_setup;
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = microbit::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    #[cfg(feature = "accel")]
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();

    #[cfg(feature = "mag")]
    sensor.set_mag_odr(MagOutputDataRate::Hz50).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    loop {
        #[cfg(feature = "accel")]
        {
            while !sensor.accel_status().unwrap().xyz_new_data {}

            let data = sensor.accel_data().unwrap();
            rprintln!("Acceleration x: {}, y: {}, z: {}", data.x, data.y, data.z);
            board.display_pins.row2.set_low().unwrap();
            timer.delay_ms(100_u16);
            board.display_pins.row2.set_high().unwrap();
        }

        #[cfg(feature = "mag")]
        {
            while !sensor.mag_status().unwrap().xyz_new_data {}

            let data = sensor.mag_data().unwrap();
            #[cfg(not(feature = "serial"))]
            rprintln!("Magnetization x: {}, y: {}, z: {}", data.x, data.y, data.z);
            #[cfg(feature = "serial")]
            write!(serial, "{};{};{}", data.x, data.y, data.z).unwrap();

            board.display_pins.row1.set_low().unwrap();
            timer.delay_ms(100_u16);
            board.display_pins.row1.set_high().unwrap();
        }

        timer.delay_ms(300_u16);
    }
}
