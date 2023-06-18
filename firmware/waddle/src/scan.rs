use crate::layout::{COLS, ROWS};

pub struct Scan {
    pressed: [u16; ROWS],
}

impl Scan {
    pub fn new() -> Self {
        Self {
            pressed: [0; ROWS]
        }
    }

    pub fn set_pressed(&mut self, row: usize, col: usize) {
        self.pressed[row] = self.pressed[row] | (1 << col)
    }

    pub fn is_pressed(&self, button: usize) -> bool {
        let c = button % COLS;
        let r = button / ROWS;
        let p = self.pressed[r] & (1 << c);
        p > 0
    }
}