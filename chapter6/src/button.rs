use embedded_hal::digital::InputPin;
use fugit::ExtU64;
use microbit::hal::gpio::{Floating, Input, Pin};
use rtt_target::rprintln;

use crate::{
    channel::Sender,
    time::{Ticker, Timer},
};

#[derive(Debug, Clone, Copy)]
pub enum ButtonDirection {
    Left,
    Right,
}

pub enum ButtonState<'a> {
    WaitForPress,
    Debounce(Timer<'a>),
}

pub struct ButtonTask<'a> {
    pin: Pin<Input<Floating>>,
    ticker: &'a Ticker,
    direction: ButtonDirection,
    state: ButtonState<'a>,
    sender: Sender<'a, ButtonDirection>,
}

impl<'a> ButtonTask<'a> {
    pub fn new(
        pin: Pin<Input<Floating>>,
        ticker: &'a Ticker,
        direction: ButtonDirection,
        sender: Sender<'a, ButtonDirection>,
    ) -> Self {
        Self {
            pin,
            ticker,
            direction,
            state: ButtonState::WaitForPress,
            sender,
        }
    }

    pub fn poll(&mut self) {
        match self.state {
            ButtonState::WaitForPress => {
                if self.pin.is_low().unwrap() {
                    rprintln!("Button pressed");
                    self.sender.send(self.direction);
                    self.state = ButtonState::Debounce(Timer::new(50.millis(), self.ticker));
                }
            }
            ButtonState::Debounce(ref timer) => {
                if timer.is_ready() && self.pin.is_high().unwrap() {
                    self.state = ButtonState::WaitForPress;
                }
            }
        }
    }
}
