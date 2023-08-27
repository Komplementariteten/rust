#![no_std]

pub mod interrupt;
pub mod panic;
mod sys;
pub mod user_led;

use core::sync::atomic::{AtomicBool, Ordering};
use cortex_m::asm;
use cortex_m::delay::Delay;
// use cortex_m::delay::Delay;
pub use cortex_m_rt::entry;
pub use hal::usb_hs::{UsbBus, USB1_ULPI as USB};
use hal::{gpio::PinState, pac, prelude::*, rcc, time::Hertz, usb_hs::USB1_ULPI};
#[allow(unused_imports)]
pub use rtt_target::{rprintln as log, rtt_init_print as log_init};
pub use stm32h7xx_hal as hal;
use stm32h7xx_hal::dac::{C1, Enabled};
use stm32h7xx_hal::hal::blocking::delay;
use stm32h7xx_hal::stm32::DAC;
use stm32h7xx_hal::traits::DacOut;
// use stm32h7xx_hal::adc;
// use stm32h7xx_hal::hal::adc::Channel;
// use stm32h7xx_hal::rcc::rec::AdcClkSel;

pub type CorePeripherals = cortex_m::Peripherals;
pub type PortentaDac  = C1<DAC, Enabled>;

pub const CORE_FREQUENCY: Hertz = Hertz::from_raw(480_000_000);

pub struct Board {
    pub led_red: user_led::Red,
    pub led_green: user_led::Green,
    pub led_blue: user_led::Blue,
    pub usb: USB1_ULPI,
    pub dac: PortentaDac,
    // pub adc_channel1: Channel<>
}



impl Board {
    pub fn take() -> Self {
        static TAKEN: AtomicBool = AtomicBool::new(false);
        debug_assert!(!TAKEN.swap(true, Ordering::SeqCst));
        Self::setup()
    }

    fn setup() -> Self {
        #[cfg(debug_assertions)]
        log!("Board init");

        // Reset previous configuration and enable external oscillator as HSE source (25 MHz)
        let cp = cortex_m::Peripherals::take().expect("Cortex Peripherals not found");
        sys::Clk::new().reset().enable_ext_clock();
        let dp = pac::Peripherals::take().expect("Other Peripherals not found");

        // Configure power domains and clock tree
        let pwrcfg = dp.PWR.constrain().vos0(&dp.SYSCFG).freeze();
        let ccdr = dp
            .RCC
            .constrain()
            .use_hse(25.MHz())
            .bypass_hse()
            .sys_ck(CORE_FREQUENCY)
            .hclk(240.MHz())
            .pll1_strategy(rcc::PllConfigStrategy::Iterative)
            .freeze(pwrcfg, &dp.SYSCFG);

        /* ADC Fix later
        let cp = cortex_m::Peripherals::take().unwrap();

        ccdr.peripheral.kernel_adc_clk_mux(AdcClkSel::Per);

        let mut delay = Delay::new(cp.SYST, ccdr.clocks.sys_ck().to_Hz());
        let mut adc1 = adc::Adc::adc1(dp.ADC1, 4.MHz(), &mut delay, ccdr.peripheral.ADC12, &ccdr.clocks).enable();
        adc1.set_resolution(adc::Resolution::SixteenBit);

        let gpiof = dp.GPIOF.split(ccdr.peripheral.GPIOF);

        let mut channel = gpioc.pc0.into_analog(); */

        debug_assert_eq!(sys::Clk::get_source(), Some(sys::ClkSource::Pll1));
        debug_assert_eq!(sys::Clk::get_pll_source(), sys::PllSourceVariant::Hse);

        // USB
        let (gpioa, gpiob, gpioc, gpioh, gpioi, gpioj) = {
            (
                dp.GPIOA.split(ccdr.peripheral.GPIOA),
                dp.GPIOB.split(ccdr.peripheral.GPIOB),
                dp.GPIOC.split(ccdr.peripheral.GPIOC),
                dp.GPIOH.split_without_reset(ccdr.peripheral.GPIOH), // Do not do a reset since external oscillator is enabled by GPIOH1
                dp.GPIOI.split(ccdr.peripheral.GPIOI),
                dp.GPIOJ.split(ccdr.peripheral.GPIOJ),
            )
        };

        #[cfg(not(feature = "rm0455"))]
            let disabled_dac = dp.DAC.dac(gpioa.pa4, ccdr.peripheral.DAC12);
        #[cfg(feature = "rm0455")]
            let disabled_dac = dp.DAC1.dac(gpioa.pa4, ccdr.peripheral.DAC1);

        // dac
        let mut delay = cp.SYST.delay(ccdr.clocks);
        let mut dac = disabled_dac.calibrate_buffer(&mut delay).enable();

        // Enable ULPI transceiver (GPIOJ4)
        let mut ulpi_reset = gpioj.pj4.into_push_pull_output();
        ulpi_reset.set_high();

        dac.set_value(2058);
        asm::bkpt();

        dac.set_value(4095);
        asm::bkpt();

        let usb = USB1_ULPI::new(
            dp.OTG1_HS_GLOBAL,
            dp.OTG1_HS_DEVICE,
            dp.OTG1_HS_PWRCLK,
            gpioa.pa5.into_alternate(),
            gpioi.pi11.into_alternate(),
            gpioh.ph4.into_alternate(),
            gpioc.pc0.into_alternate(),
            gpioa.pa3.into_alternate(),
            gpiob.pb0.into_alternate(),
            gpiob.pb1.into_alternate(),
            gpiob.pb10.into_alternate(),
            gpiob.pb11.into_alternate(),
            gpiob.pb12.into_alternate(),
            gpiob.pb13.into_alternate(),
            gpiob.pb5.into_alternate(),
            ccdr.peripheral.USB1OTG,
            &ccdr.clocks,
        );

        // User LEDs
        let gpiok = dp.GPIOK.split(ccdr.peripheral.GPIOK);
        let (led_red, led_green, led_blue) = (
            gpiok.pk5.into_push_pull_output_in_state(PinState::High),
            gpiok.pk6.into_push_pull_output_in_state(PinState::High),
            gpiok.pk7.into_push_pull_output_in_state(PinState::High),
        );

        Board {
            led_red,
            led_green,
            led_blue,
            usb,
            dac,
        }
    }
}