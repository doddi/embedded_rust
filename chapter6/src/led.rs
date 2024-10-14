use crate::{
    button::ButtonDirection,
    channel::Receiver,
    time::{Ticker, Timer},
};
use embedded_hal::digital::{OutputPin, StatefulOutputPin};
use fugit::ExtU64;
use microbit::{
    gpio::{DisplayPins, NUM_COLS, NUM_ROWS},
    hal::gpio::{Output, Pin, PushPull},
};
use rtt_target::rprintln;

enum LedState<'a> {
    Toggle,
    Wait(Timer<'a>),
}

#[allow(clippy::upper_case_acronyms)]
type LED = Pin<Output<PushPull>>;

pub struct LedTask<'a> {
    display: ([LED; NUM_COLS], [LED; NUM_ROWS]),
    active_col: usize,
    ticker: &'a Ticker,
    led_state: LedState<'a>,
    receiver: Receiver<'a, ButtonDirection>,
}

impl<'a> LedTask<'a> {
    pub fn new(
        display: DisplayPins,
        ticker: &'a Ticker,
        receiver: Receiver<'a, ButtonDirection>,
    ) -> Self {
        let (col, mut row) = display.degrade();
        row[0].set_high().ok();
        Self {
            display: (col, row),
            active_col: 0,
            ticker,
            led_state: LedState::Toggle,
            receiver,
        }
    }

    pub fn poll(&mut self) {
        match self.led_state {
            LedState::Toggle => {
                let activate = self.convert_display(self.active_col);
                rprintln!("{:?}", activate);
                self.display.0[activate.0].toggle().ok();
                // self.display.0[3].toggle().ok();
                self.led_state = LedState::Wait(Timer::new(500.millis(), self.ticker));
            }
            LedState::Wait(ref timer) => {
                if timer.is_ready() {
                    self.led_state = LedState::Toggle;
                }

                if let Some(direction) = self.receiver.receive() {
                    self.shift(direction);
                }
            }
        }
    }

    fn convert_display(&mut self, col: usize) -> (usize, usize) {
        let top_row: [(usize, usize); 5] = [(0, 0), (3, 1), (1, 0), (4, 1), (2, 0)];

        top_row[col]
    }

    fn shift(&mut self, direction: ButtonDirection) {
        rprintln!("Button direction: {:?}", direction);

        // turn off current led
        let activate = self.convert_display(self.active_col);
        self.display.0[activate.0].set_low().unwrap();
        self.display.1[activate.1].set_low().unwrap();

        // go to the next column
        self.active_col = match direction {
            ButtonDirection::Left => match self.active_col {
                0 => 4,
                _ => self.active_col - 1,
            },
            ButtonDirection::Right => (self.active_col + 1) % 5,
        };

        let activate = self.convert_display(self.active_col);
        self.display.1[activate.1].set_high().unwrap();
    }
}
