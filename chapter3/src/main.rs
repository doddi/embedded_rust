#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nrf52811_pac::Peripherals;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Chapter 3");

    let peripherals = Peripherals::take().unwrap();
    peripherals.P0.pin_cnf[4].write(|writer| writer.dir().output());
    peripherals.P0.pin_cnf[13].write(|writer| writer.dir().output());

    let mut is_on = false;
    loop {
        peripherals.P0.out.write(|writer| writer.pin13().bit(is_on));
        for _ in 0..50_000 {
            cortex_m::asm::nop();
        }

        rprintln!("LED is {}", if is_on { "on" } else { "off" });
        is_on = !is_on;
    }
}
