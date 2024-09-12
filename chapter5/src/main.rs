#![no_std]
#![no_main]

use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::board::Board;
use microbit::hal::Timer;

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Chapter 5");

    let mut board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);

    board.display_pins.col1.set_low().unwrap();
    let mut row1 = board.display_pins.row1;

    loop {
        let _ = row1.set_low();
        timer.delay_ms(1_000);
        let _ = row1.set_high();
        timer.delay_ms(1_000);
    }
}
