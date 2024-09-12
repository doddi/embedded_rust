#![no_std]
#![no_main]

use embedded_hal::digital::InputPin;
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

    let mut button = gpio.p0_17.into_pullup_input();

    // Led is connected to pin 04 and 13
    let _ = gpio.p0_04.into_push_pull_output(Level::Low);
    let mut led = gpio.p0_13.into_push_pull_output(Level::High);

    loop {
        if button.is_high().unwrap() {
            rprintln!("Button is not pressed");
            led.set_low().unwrap();
        } else {
            rprintln!("Button is pressed");
            led.set_high().unwrap();
        }
    }
}
