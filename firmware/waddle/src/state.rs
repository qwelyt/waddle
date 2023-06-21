use heapless::Vec;

use crate::event::Event;
use crate::keyboard::DELAY_MS;
use crate::layout::{BUTTONS, Key, LAYERS, LAYOUT, LEDS};
use crate::position::position::Position;
use crate::scan::Scan;
use crate::vec;

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
                self.pressed[i] = self.pressed[i].saturating_add(1);
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
        // let mut buttons: Vec<Position, BUTTONS> = Vec::new();
        let mut buttons: [Option<Position>; BUTTONS] = [None; BUTTONS];
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
                // buttons.push(Position::from(i));
                buttons[i] = Some(Position::from(i));
            }
        }
        // // 1. Find which layer we are on
        let layer: u8 = buttons.iter()
            .map(|o| *o)
            .filter(Option::is_some)
            .map(Option::unwrap)
            .map(|p| LAYOUT.get_layer_mod(&p))
            .sum();

        // // 2. Get all keys on that layer, or lower if current is PassThrough
        // 3. Add KeyCodes and Functions as events
        // 4. Check if on-holds. If they are above limit, get the key.
        //      If they are below, ignore.
        let events: Vec<Event, BUTTONS> = buttons.iter()
            .map(|o| *o)
            .filter(Option::is_some)
            .map(Option::unwrap)
            .map(|p| self.get_key(&p, layer))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();

        events
    }
    fn ms_to_ticks(ms: u8) -> u8 {
        ms / DELAY_MS as u8
    }
    fn get_key(&self, position: &Position, layer: u8) -> Option<Event> {
        match LAYOUT.get_key(layer, position) {
            Key::KeyCode(kc) => Some(Event::KeyCode(kc)),
            // Key::KeyCode(kc) => None,
            Key::PassThrough(go_down) => self.get_key(position, layer - go_down),
            _ => None
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