use hash32::{BuildHasherDefault, FnvHasher};
use heapless::{FnvIndexSet, IndexSet, Vec};

use crate::layout::{BUTTONS, COLS, Key, LAYERS, ROWS};
use crate::position::position::Position;

#[derive(Clone, Debug)]
pub struct State {
    pressed: [bool; BUTTONS],
}

impl State {
    pub fn empty() -> Self {
        Self {
            pressed: [false; BUTTONS]
        }
    }

    pub fn new(pressed: [bool; BUTTONS]) -> Self {
        Self {
            pressed,
        }
    }

    pub fn set_pressed(&mut self, row: usize, col: usize) {
        // (0,6) -> 6
        // (1,6) -> 1 * COLS + 6 -> 1*12+6 -> 18
        // (3,12) -> 3*12+12 = 36+12 -> 48
        let p = row * COLS + col;
        self.pressed[p] = true;
    }

    pub fn pressed(&self) -> Vec<Position, BUTTONS> {
        let mut v = Vec::new();
        for (i, b) in self.pressed.iter().enumerate() {
            if *b {
                let row = i / COLS;
                let col = i % COLS;
                v.push(Position::new(u8::try_from(row).unwrap(), u8::try_from(col).unwrap()));
            }
        }
        v
    }
}

impl PartialEq<Self> for State {
    fn eq(&self, other: &Self) -> bool {
        self.pressed.eq(&other.pressed)
    }
}

impl Eq for State {}