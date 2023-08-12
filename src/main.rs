#![no_main]
#![no_std]

use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use defmt_rtt as _; // global logger
use nrf52840_hal as _; // memory layout
use nrf52840_hal as hal;
use nrf52840_hal::gpio::Level;

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let button = port0.p0_13.into_pullup_input();
    let mut led = port0.p0_17.into_push_pull_output(Level::Low);
    defmt::info!("Hello, World!");
    loop {
        if button.is_high().unwrap() {
            led.set_high().unwrap();
            defmt::info!("Button high!");
        } else {
            led.set_low().unwrap();
            defmt::info!("Button low!");
        }
    }
}

// #![no_std]
// #![no_main]

// extern crate panic_semihosting;

// use cortex_m_rt::entry;

// use nrf52840_hal::{
//     gpio::*,
//     pac,
//     twim::{self, Twim},
// };

// pub mod ssd1306;
// #[entry]
// fn main() -> ! {
//     let p = pac::Peripherals::take().unwrap();
//     let port0 = p0::Parts::new(p.P0);

//     let scl = port0.p0_26.into_floating_input().degrade();
//     let sda = port0.p0_27.into_floating_input().degrade();
//     let pins = twim::Pins { scl, sda };

//     ssd1306::test_display(pins);

//     loop {}
// }