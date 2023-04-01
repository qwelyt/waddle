use hash32::{BuildHasherDefault, FnvHasher};
use heapless::{FnvIndexSet, IndexSet, Vec};

use crate::layout::{BUTTONS, COLS, Key, LAYERS, ROWS};
use crate::position::position::Position;

#[derive(Clone, Debug)]
pub struct State {
    pressed: IndexSet<Position, BuildHasherDefault<FnvHasher>, 64>,
}

impl State {
    pub fn empty() -> Self {
        Self {
            pressed: FnvIndexSet::new(),
        }
    }
    pub fn new(pressed: FnvIndexSet<Position, 64>) -> Self {
        Self {
            pressed,
        }
    }

    pub fn pressed(&self) -> Vec<Position, BUTTONS> {
        let mut v = Vec::new();
        for p in self.pressed.iter() {
            v.push(*p);
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