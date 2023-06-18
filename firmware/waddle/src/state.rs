use heapless::Vec;

use crate::event::Event;
use crate::keyboard::DELAY_MS;
use crate::layout::{BUTTONS, Key, LAYERS, LAYOUT, LEDS};
use crate::position::position::Position;
use crate::scan::Scan;

pub struct State {
    pressed: [u8; BUTTONS],
    released: [u8; BUTTONS],
    leds: u8,
}

impl State {
    pub fn new() -> Self {
        Self {
            pressed: [0; BUTTONS],
            released: [0; BUTTONS],
            leds: 0,
        }
    }

    pub fn tick(&mut self, scan: &Scan) {
        for i in 0..BUTTONS {
            self.released[i] = 0; // Reset released

            if scan.is_pressed(i) {
                let cur_val = self.pressed[i];
                let new_val = cur_val.saturating_add(1);
                self.pressed[i] = new_val;
                // self.pressed[i] = 0;
            } else {
                // It's either 0 as it was never pressed which means there is no change
                // Or it was pressed and we should check for how long it was pressed to check for
                // on-holds.
                self.released[i] = self.pressed[i];
                self.pressed[i] = 0;
            }
        }
    }


    pub fn events(&mut self) -> Vec<Event, BUTTONS> {
        let mut buttons: Vec<Position, BUTTONS> = Vec::new();
        for i in 0..BUTTONS {
            let rticks = self.released[i];
            if rticks > 2 {
                // Check for on-holds. If ticks are below the on-hold limit then
                // send the non-hold key as an event.
                // If the ticks are *over* then we have already activated that key
                // and should do nothing
            }

            let pticks = self.pressed[i];
            if pticks > 2 {
                buttons.push(Position::from(i));
            }
        }
        // // 1. Find which layer we are on
        let layer: u8 = buttons.iter()
            .map(|p| LAYOUT.get_layer_mod(p))
            .sum();

        // // 2. Get all keys on that layer, or lower if current is PassThrough
        let keys: Vec<Key, BUTTONS> = buttons.iter()
            .map(|p| LAYOUT.get_key(layer, p))
            .collect();

        // 3. Add KeyCodes and Functions as events
        // 4. Check if on-holds. If they are above limit, get the key.
        //      If they are below, ignore.
        let events: Vec<Event, BUTTONS> = keys.iter()
            .map(|k| match k {
                Key::KeyCode(kc) => Some(Event::KeyCode(*kc)),
                // Key::Function(f) => Some(Event::Function(*f)), // Why not just ... apply directly?
                Key::Function(f) => {
                    f(self);
                    None
                }
                _ => None
            })
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();

        events
    }
    fn ms_to_ticks(ms: u8) -> u8 {
        ms / DELAY_MS as u8
    }
    fn get_key(&self, keys: &[Key; 4], layer: u8) -> Option<Key> {
        match keys[layer as usize] {
            Key::Dead => None,
            Key::PassThrough(go_down) => self.get_key(keys, layer - go_down),
            _ => Some(keys[layer as usize])
        }
    }


    pub fn toggle_led(&mut self, led: u8) {
        self.leds = self.leds ^ (1 << led)
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