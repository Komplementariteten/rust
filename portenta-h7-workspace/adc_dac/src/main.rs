//! Example Blinky
//!
//! Toggles the 3 user LEDs with a different frequency
//!

#![no_std]
#![no_main]

#[allow(unused_imports)]
use portenta_h7::{log, log_init, user_led, PortentaDac};
use rtic::app;
use systick_monotonic::{fugit::MillisDurationU64, Systick};
use cortex_m::asm;
use stm32h7xx_hal::hal::Direction;
use stm32h7xx_hal::traits::DacOut;

#[app(device = portenta_h7::hal::pac, peripherals = false, dispatchers = [SPI1])]
mod app {
    use super::*;

    #[shared]
    struct Shared {
    }

    #[local]
    struct Local {
        led_red: user_led::Red,
        dac: PortentaDac
    }

    #[monotonic(binds = SysTick, default = true)]
    type MonoTimer = Systick<1_000>;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        log_init!();

        let mono = Systick::new(cx.core.SYST, portenta_h7::CORE_FREQUENCY.raw());

        // Get boatd resources
        let portenta_h7::Board {
            led_red,
            dac,
            ..
        } = portenta_h7::Board::take();

        #[cfg(debug_assertions)]
        log!("spawning tasks");
        let _ = blink_led_red::spawn();
        let dac_value = 4095;
        let _ = set_dac::spawn(dac_value, Direction::Downcounting);
        (
            Shared {
            },
            Local {
                led_red,
                dac,
            },
            init::Monotonics(mono),
        )
    }

    #[task(local = [dac])]
    fn set_dac(cx: set_dac::Context, value: u16, dir:Direction) {
        #[cfg(debug_assertions)]
        cx.local.dac.set_value(value);
        log!("dac {}", value);
        let new_dir = match value {
            0 => Direction::Upcounting,
            4095 => Direction::Downcounting,
            _ => dir,
        };
        let new_value = match new_dir {
            Direction::Upcounting => value + 1,
            Direction::Downcounting => value -1,
        };

        asm::bkpt();
        set_dac::spawn_after(MillisDurationU64::from_ticks(200), new_value, new_dir).unwrap();
    }

    #[task(local = [led_red])]
    fn blink_led_red(cx: blink_led_red::Context) {
        #[cfg(debug_assertions)]
        log!("toggling {:?}", cx.local.led_red);
        cx.local.led_red.toggle();
        blink_led_red::spawn_after(MillisDurationU64::from_ticks(500)).unwrap();
    }
}