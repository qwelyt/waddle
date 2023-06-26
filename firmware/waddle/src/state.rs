use core::cmp::max_by_key;

use heapless::Vec;

use crate::keyboard::DELAY_MS;
use crate::layout::{BUTTONS, Key, KeyType, LAYERS, LAYOUT, LEDS};
use crate::position::position::Position;
use crate::scan::Scan;
use crate::state::ButtonState::{Pressed, Released};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ButtonState {
    Released,
    Pressed,
    Held,
}

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

    pub fn tick(&mut self, scan: &Scan) -> [ButtonState; 48] {
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
        let mut button_state = [Released; BUTTONS];
        // let layer = self.layer();
        for i in 0..BUTTONS {
            // let key = LAYOUT.get_key()
            // ToDo: Handle Hold
            if self.pressed[i] > 2 {
                button_state[i] = Pressed;
            } else {
                button_state[i] = Released;
            }
        }
        button_state
    }

    pub fn keys(&self) -> Vec<Key, BUTTONS> {
        // Check for on-holds. If ticks are below the on-hold limit then
        // send the non-hold key as an event.
        // If the ticks are *over* then we have already activated that key
        // and should do nothing
        // // 1. Find which layer we are on
        // // 2. Get all keys on that layer, or lower if current is PassThrough
        // 3. Add KeyCodes and Functions as events
        // 4. Check if on-holds. If they are above limit, get the key.
        //      If they are below, ignore.
        let layer = self.layer();

        let keys: Vec<Key, BUTTONS> = self.pressed.iter().enumerate()
            .filter(|(i, v)| **v >= 2)
            .map(|(i, v)| (Position::from(i), v))
            .map(|(p, v)| State::get_key(&p, layer, *v))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();
        keys
    }

    fn layer(&self) -> u8 {
        // self.pressed.iter().enumerate()
        //     .filter(|(i, v)| **v >= 2)
        //     .map(|(i, v)| Position::from(i))
        //     .map(|p| LAYOUT.get_layer_mod(&p))
        //     .sum()
        self.pressed.iter().enumerate()
            .filter(|(i, v)| **v >= 2)
            .map(|(i, v)| (Position::from(i), v))
            .map(|(p, v)| State::get_key(&p, 0, *v))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .map(|k| match k {
                Key::LayerMo(layer) => layer,
                _ => 0
            })
            .sum()
    }

    fn ms_to_ticks(ms: u8) -> u8 {
        ms / DELAY_MS as u8
    }

    fn get_key(position: &Position, layer: u8, hold_time: u8) -> Option<Key> {
        // match LAYOUT.get_key(layer, position) {
        //     Key::KeyCode(kc) => Some(Key::KeyCode(kc)),
        //     Key::Function(f) => Some(Key::Function(f)),
        //     Key::PassThrough(go_down) => State::get_key(position, layer - go_down),
        //     _ => None
        // }
        match LAYOUT.get_key(layer, position) {
            KeyType::Instant(key) => State::get_instant_key(key, position, layer, hold_time),
            KeyType::OnHold(key1, hold_limit, key2) => State::get_hold_key(key1, hold_limit, key2, position, layer, hold_time)
        }
    }

    fn get_instant_key(key: Key, position: &Position, layer: u8, hold_time: u8) -> Option<Key> {
        match key {
            Key::KeyCode(kc) => Some(Key::KeyCode(kc)),
            Key::Function(f) => Some(Key::Function(f)),
            Key::PassThrough(go_down) => State::get_key(position, layer - go_down, hold_time),
            Key::LayerMo(l) => Some(Key::LayerMo(l)),
            _ => None
        }
    }
    fn get_hold_key(key1: Key, hold_limit: u8, key2: Key, position: &Position, layer: u8, hold_time: u8) -> Option<Key> {
        let k = if hold_limit > hold_time { key1 } else { key2 };
        State::get_instant_key(k, position, layer, hold_time)
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