use heapless::Vec;

use crate::layout::{BUTTONS, COLS, ROWS};
use crate::position::position::Position;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pressed: [u16; ROWS],
}

impl State {
    pub fn empty() -> Self {
        Self {
            pressed: [0; ROWS]
        }
    }

    pub fn new(pressed: [u16; ROWS]) -> Self {
        Self {
            pressed,
        }
    }
    pub fn intersect(a: State, b: State) -> State {
        // State::new(a.pressed & b.pressed)
        let mut intersect = [0; ROWS];
        for r in 0..ROWS {
            intersect[r] = a.pressed[r] & b.pressed[r];
        }
        State::new(intersect)
    }

    pub fn set_pressed(&mut self, row: usize, col: usize) {
        // (0,6) -> 6
        // (1,6) -> 1 * COLS + 6 -> 1*12+6 -> 18
        // (3,12) -> 3*12+12 = 36+12 -> 48
        // let p = row * COLS + col;
        // let bit = 1 << p;
        // self.pressed = self.pressed | bit;
        self.pressed[row] = self.pressed[row] | (1 << col)
    }

    pub fn pressed(&self) -> Vec<Position, BUTTONS> {
        let mut v = Vec::new();
        // for i in 0..BUTTONS {
        //     let p = self.pressed & (1 << i);
        //     if p > 0 {
        //         let row = i / COLS;
        //         let col = i % COLS;
        //         v.push(Position::new(u8::try_from(row).unwrap(), u8::try_from(col).unwrap()));
        //     }
        // }
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
}