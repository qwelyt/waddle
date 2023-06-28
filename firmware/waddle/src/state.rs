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
    pub fn init(&mut self) {
        for i in 0..BUTTONS {
            self.released[i] = u8::MAX;
        }
    }

    pub fn tick(&mut self, scan: &Scan) -> [ButtonState; BUTTONS] {
        for i in 0..BUTTONS {
            // The key is either pressed or the key is released.
            // We want to know for how long the key has been in each state since the last change.
            // All keys start off as released TO THE MAX, and then usage changes everything.
            // The pressed_time will help us know when to activate for example OnHolds
            // The released_time will help us if we implement DoubleTap. Or TapHold.
            // A TapHold checks for a short release and a long press.
            // A DoubleTap check for a short release and a short press.
            if scan.is_pressed(i) {
                self.pressed[i] = self.pressed[i].saturating_add(1);
                self.released[i] = 0;
            } else {
                self.released[i] = self.released[i].saturating_add(1);
                self.pressed[i] = 0;
            }
        }
        let mut button_state = [Released; BUTTONS];
        // let layer = self.layer();
        for i in 0..BUTTONS {
            if self.pressed[i] > 2 {
                button_state[i] = Pressed;
            } else if self.released[i] > 2 {
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

        // TODO: As we are now dealing with on-holds, we can only send key presses for on-holds if they
        // are either released or past the on-hold time. If a key is on-hold, and we are under the time limit
        // then we can't send any keys. Perhaps we are just waiting for it to activate? Then we can't send key1.
        // Only if we are past wait_time can we send key2, and only if we have release the
        // key can we send key1 if we are under wait_time.
        let layer = self.layer();

        // A key is relevant if it is pressed or if it is being held.
        // This is true for Instants.
        // For OnHold a key is relevant if it is held OR just released.
        // If a key is NOT OnHold, then we should register release as release of the button, simple
        // If it IS OnHold, the key is relevant and we need to check
        //      1) Is the key still being held?
        //          1.1) If over hold_limit then we send key2
        //          1.2) If under hold_limit we do nothing
        //      2) Is the key released?
        //          2.1) If release_time is >1 tick it's dead. Blank report.
        //          2.2) If release_time ti <2 tick check hold_time.
        //              2.2.1) If over hold_limit it's dead. Blank report.
        //              2.2.2) If under hold_limit send key1. Next tick will blank it.

        let keys: Vec<Key, BUTTONS> = self.pressed.iter().enumerate()
            .filter(|(i, hold_time)| **hold_time >= 2)
            .map(|(i, hold_time)| (Position::from(i), hold_time, self.released[i]))
            .map(|(p, hold_time, release_time)| State::get_key(&p, layer, *hold_time, release_time))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();
        keys
    }

    fn layer(&self) -> u8 {
        self.pressed.iter().enumerate()
            .filter(|(i, hold_time)| **hold_time >= 2)
            .map(|(i, hold_time)| (Position::from(i), hold_time, self.released[i]))
            .map(|(p, hold_time, release_time)| State::get_key(&p, 0, *hold_time, release_time))
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

    fn get_key(position: &Position, layer: u8, hold_time: u8, release_time: u8) -> Option<Key> {
        match LAYOUT.get_key(layer, position) {
            KeyType::Instant(key) => State::get_instant_key(key, position, layer, hold_time, release_time),
            KeyType::OnHold(key1, hold_limit, key2) => State::get_hold_key(key1, hold_limit, key2, position, layer, hold_time, release_time)
        }
    }

    fn get_instant_key(key: Key, position: &Position, layer: u8, hold_time: u8, release_time: u8) -> Option<Key> {
        match key {
            Key::KeyCode(kc) => Some(Key::KeyCode(kc)),
            Key::Function(f) => Some(Key::Function(f)),
            Key::PassThrough(go_down) => State::get_key(position, layer - go_down, hold_time, release_time),
            Key::LayerMo(l) => Some(Key::LayerMo(l)),
            _ => None
        }
    }
    fn get_hold_key(key1: Key, hold_limit: u8, key2: Key, position: &Position, layer: u8, hold_time: u8, release_time: u8) -> Option<Key> {
        // // If the key is pressed, but hold_time is less than hold_limit then send no key.
        // // If the key is pressed and hold_time is greater than hold_limit send key2
        // // If the key is released and hold_time WAS less than hold_limit send key1
        return if hold_time < hold_limit {
            // None
            State::get_instant_key(key1, position, layer, hold_time, release_time)
        } else if hold_time >= hold_limit {
            State::get_instant_key(key2, position, layer, hold_time, release_time)
        } else {
            // SHOULD be the only option left, right?
            // GAH! The button isn't pressed anymore when we currently achive this state, to it's
            // never triggered! Need to scan all keys, not just pressed ones to make sure that we
            // can actually trigger here
            State::get_instant_key(key1, position, layer, hold_time, release_time)
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
