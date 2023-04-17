use heapless::Vec;

use crate::layout::{BUTTONS, COLS, LEDS, ROWS};
use crate::position::position::Position;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pressed: [u16; ROWS],
    leds: u8,
}

impl State {
    pub fn empty() -> Self {
        Self {
            pressed: [0; ROWS],
            leds: 0,
        }
    }

    pub fn new(pressed: [u16; ROWS], leds: u8) -> Self {
        Self { pressed, leds }
    }

    pub fn clean(&self) -> Self {
        Self {
            pressed: [0; ROWS],
            leds: self.leds,
        }
    }

    pub fn intersect(a: State, b: State) -> State {
        let mut intersect = [0; ROWS];
        for r in 0..ROWS {
            intersect[r] = a.pressed[r] & b.pressed[r];
        }
        State::new(intersect, a.leds)
    }

    pub fn set_pressed(&mut self, row: usize, col: usize) {
        self.pressed[row] = self.pressed[row] | (1 << col)
    }

    pub fn toggle_led(&mut self, led: u8) {
        self.leds = self.leds ^ (1 << led)
    }

    pub fn pressed(&self) -> Vec<Position, BUTTONS> {
        let mut v = Vec::new();
        for r in 0..ROWS {
            for c in 0..COLS {
                let p = self.pressed[r] & (1 << c);
                if p > 0 {
                    v.push(Position::new(u8::try_from(r).unwrap(), u8::try_from(c).unwrap()));
                }
            }
        }
        v
    }

    pub fn led_state(&self) -> [bool; LEDS] {
        let mut leds = [false; LEDS];
        for i in 0..LEDS {
            let l = self.leds & (1 << i);
            if l > 0 {
                leds[i] = true;
            }
        }
        leds
    }
}