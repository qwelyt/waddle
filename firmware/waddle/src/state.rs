use heapless::Vec;

use crate::layout::{BUTTONS, COLS};
use crate::position::position::Position;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pressed: usize,
}

impl State {
    pub fn empty() -> Self {
        Self {
            pressed: 0
        }
    }

    pub fn new(pressed: usize) -> Self {
        Self {
            pressed,
        }
    }
    pub fn intersect(a: State, b: State) -> State {
        State::new(a.pressed & b.pressed)
    }

    pub fn set_pressed(&mut self, row: usize, col: usize) {
        // (0,6) -> 6
        // (1,6) -> 1 * COLS + 6 -> 1*12+6 -> 18
        // (3,12) -> 3*12+12 = 36+12 -> 48
        let p = row * COLS + col;
        let bit = 1 << p;
        self.pressed = self.pressed | bit;
    }

    pub fn pressed(&self) -> Vec<Position, BUTTONS> {
        let mut v = Vec::new();
        for i in 0..BUTTONS {
            let p = self.pressed & (1 << i);
            if p > 0 {
                let row = i / COLS;
                let col = i % COLS;
                v.push(Position::new(u8::try_from(row).unwrap(), u8::try_from(col).unwrap()));
            }
        }
        v
    }
}