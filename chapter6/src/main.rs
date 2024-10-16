#![no_std]
#![no_main]

mod button;
mod channel;
mod led;
mod time;

use button::{ButtonDirection, ButtonTask};
use channel::Channel;
use led::LedTask;
use microbit::board::Board;
use time::Ticker;

use cortex_m_rt::entry;
use nrf51_hal::Clocks;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Chapter 6");

    let board = Board::take().unwrap();

    Clocks::new(board.CLOCK).start_lfclk();

    let ticker = Ticker::new(board.RTC0);
    let channel: Channel<ButtonDirection> = Channel::new();

    let mut led_task = LedTask::new(board.display_pins, &ticker, channel.get_receiver());

    let button_left = board.buttons.button_a.degrade();
    let button_right = board.buttons.button_b.degrade();

    let mut button_left_task = ButtonTask::new(
        button_left,
        &ticker,
        ButtonDirection::Left,
        channel.get_sender(),
    );
    let mut button_right_task = ButtonTask::new(
        button_right,
        &ticker,
        ButtonDirection::Right,
        channel.get_sender(),
    );

    loop {
        led_task.poll();
        button_left_task.poll();
        button_right_task.poll();
    }
}
