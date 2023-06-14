use heapless::{FnvIndexMap, Vec};

use crate::layout::{BUTTONS, Key, LAYOUT};
use crate::layout::Key::OnHold;
use crate::position::position::Position;
use crate::state::State;

pub struct Active {
    timers: FnvIndexMap<Position, (Key, Key), 8>,
    combos: FnvIndexMap<Position, FnvIndexMap<Position, (Key, Key), 8>, 8>,
    basic: Vec<Key, BUTTONS>,
}

impl Active {
    fn new() -> Self {
        Self {
            timers: FnvIndexMap::new(),
            combos: FnvIndexMap::new(),
            basic: Vec::new(),
        }
    }
    pub fn update(&mut self, state: &State) {
        // All pressed keys on base layer
        let l0: Vec<Key, BUTTONS> = state.pressed()
            .iter()
            .map(|p| LAYOUT.get_key(0, p))
            .collect();

        let on_holds: Vec<_, _> = l0.iter()
            .filter(|k| matches!(k, Key::OnHold(_,_)))
            .collect();

        let combos: Vec<_, _> = l0.iter()
            .filter(|k| matches!(k, Key::Combo(_,_)))
            .collect();


        // Get layer TODO: what if layer-key is behind on-hold? Or Combo?
        let layer = state.pressed()
            .iter()
            .map(|p| LAYOUT.get_layer_mod(p))
            .sum();

        let unsorted_active: Vec<(Position, Key), BUTTONS> = state.pressed().iter()
            .map(|p| (*p, LAYOUT.get_key(layer, p)))
            .collect();


        // Update timers
        self.update_timers(state);
        // Update combos
        self.update_combos(state);
        // Update basic
        self.update_basic(state);
    }


    pub fn get(self) -> Vec<Key, BUTTONS> {}
}