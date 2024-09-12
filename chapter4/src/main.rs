#![no_std]
#![no_main]

use embedded_hal::digital::OutputPin;
use nrf51_hal as hal;
use nrf51_hal::gpio::Level;

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Chapter 4");

    let pac = hal::pac::Peripherals::take().unwrap();
    let gpio = hal::gpio::p0::Parts::new(pac.GPIO);

    // Led is connected to pin 04 and 13
    let _ = gpio.p0_04.into_push_pull_output(Level::Low);
    let mut led = gpio.p0_13.into_push_pull_output(Level::High);

    let mut is_on = false;
    loop {
        if is_on {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }

        for _ in 0..50_000 {
            cortex_m::asm::nop();
        }
        is_on = !is_on;
    }
}
