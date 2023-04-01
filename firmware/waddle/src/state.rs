use hash32::{BuildHasherDefault, FnvHasher};
use heapless::{FnvIndexSet, IndexSet};

use crate::position::position::Position;

#[derive(Clone, Debug)]
pub struct State {
    layer: u8,
    pressed: IndexSet<Position, BuildHasherDefault<FnvHasher>, 64>,
}


impl State {
    pub fn empty() -> Self {
        Self {
            layer: 0,
            pressed: FnvIndexSet::new(),
        }
    }
    pub fn new(layer: u8, pressed: FnvIndexSet<Position, 64>) -> Self {
        Self {
            layer,
            pressed,
        }
    }
}

impl PartialEq<Self> for State {
    fn eq(&self, other: &Self) -> bool {
        self.layer == other.layer && self.pressed.eq(&other.pressed)
    }
}

impl Eq for State {}