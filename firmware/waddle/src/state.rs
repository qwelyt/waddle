use core::cmp::max_by_key;
use core::ops::Deref;

use heapless::Vec;

use crate::{rvec, vec};
use crate::keyboard::DELAY_MS;
use crate::layout::{BUTTONS, Key, KeyType, LAYERS, LAYOUT, LEDS};
use crate::position::position::Position;
use crate::scan::Scan;
use crate::state::ButtonState::{JustPressed, JustReleased, Pressed, Released};

fn ms_to_ticks(ms: u8) -> u8 {
    ms / DELAY_MS as u8
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ButtonState {
    Released,
    Pressed,
    JustPressed,
    JustReleased,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Transition {
    ReleasedToPressed,
    PressedToReleased,
    PressedToHeld,
    HeldToReleased,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Time {
    pressed: u8,
    released: u8,
}

impl Time {
    fn new() -> Self { Self { pressed: 0, released: 0 } }
    fn pressed(&mut self) { self.pressed = self.pressed.saturating_add(1); }
    fn released(&mut self) { self.released = self.released.saturating_add(1); }
}

struct Button {
    state: ButtonState,
    time: Time,
}

impl Button {
    fn new() -> Self { Self { state: Released, time: Time::new() } }
    fn released(&mut self) {
        if self.state == Pressed {
            self.time.released = 0;
        }
        self.state = Released;
        self.time.released();
    }
    fn pressed(&mut self) {
        if self.state == Released {
            self.time.pressed = 0;
        }
        self.state = Pressed;
        self.time.pressed();
    }

    fn is_pressed(&self) -> bool {
        self.state == Pressed && self.time.pressed > 2
    }
}

pub struct State {
    keys: Vec<Button, BUTTONS>,
    leds: u8,
}

impl State {
    pub fn new() -> Self {
        Self {
            keys: rvec![Button::new(), BUTTONS],
            leds: 0,
        }
    }
    pub fn init(&mut self) {
        for i in 0..BUTTONS {
            // self.keys[i] = Time::new();
            // self.keys[i] = (0, u8::MAX);
            // self.keys[i].1 = u8::MAX;
        }
    }

    pub fn tick(&mut self, scan: &Scan) -> [ButtonState; BUTTONS] {
        // The key is either pressed or the key is released.
        // We want to know for how long the key has been in each state since the last change.
        // The pressed_time will help us know when to activate for example OnHolds
        // The released_time will help us if we implement DoubleTap. Or TapHold.
        // A TapHold checks for a short release and a long press.
        // A DoubleTap check for a short release and a short press.

        self.keys.iter_mut().enumerate()
            .for_each(|(i, key)| match scan.is_pressed(&i) {
                true => key.pressed(),
                false => key.released(),
            });
        let layer = self.layer();
        let mut button_state = [Released; BUTTONS];
        button_state.iter_mut().enumerate()
            .for_each(|(i, bs)| {
                let k = &self.keys[i];
                *bs = match k.is_pressed() {
                    true => {
                        let key_type = LAYOUT.get_key(layer, &Position::from(i));
                        match key_type {
                            KeyType::Instant(_) => Pressed,
                            KeyType::OnHold(_, limit, _) => match k.time.pressed < ms_to_ticks(limit) { // This needs to match that keys on-hold time
                                true => JustPressed,
                                false => Pressed,
                            }
                        }
                    },
                    false => match k.time.released < 2 {
                        true => JustReleased,
                        false => Released,
                    }
                }
            });
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

        let keys: Vec<Key, BUTTONS> = self.keys.iter().enumerate()
            .map(|(i, button)| (Position::from(i), button))
            .map(|(p, button)| State::get_key(&p, layer, button))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();
        keys
    }

    fn layer(&self) -> u8 {
        self.keys.iter().enumerate()
            .filter(|(i, button)| button.time.pressed >= 2)
            .map(|(i, button)| (Position::from(i), button))
            .map(|(p, button)| State::get_key(&p, 0, button))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .map(|k| match k {
                Key::LayerMo(layer) => layer,
                _ => 0
            })
            .sum()
    }


    fn get_key(position: &Position, layer: u8, button: &Button) -> Option<Key> {
        match LAYOUT.get_key(layer, position) {
            KeyType::Instant(key) => State::get_instant_key(key, position, layer, button),
            KeyType::OnHold(key1, hold_limit, key2) => State::get_hold_key(key1, hold_limit, key2, position, layer, button)
        }
    }

    fn get_instant_key(key: Key, position: &Position, layer: u8, button: &Button) -> Option<Key> {
        match button.is_pressed() {
            true =>
                match key {
                    Key::KeyCode(kc) => Some(Key::KeyCode(kc)),
                    Key::Function(f) => Some(Key::Function(f)),
                    Key::PassThrough(go_down) => State::get_key(position, layer - go_down, button),
                    Key::LayerMo(l) => Some(Key::LayerMo(l)),
                    _ => None
                },
            false => None
        }
    }
    fn get_hold_key(key1: Key, hold_limit: u8, key2: Key, position: &Position, layer: u8, button: &Button) -> Option<Key> {
        // // If the key is pressed, but hold_time is less than hold_limit then send no key.
        // // If the key is pressed and hold_time is greater than hold_limit send key2
        // // If the key is released and hold_time WAS less than hold_limit send key1
        match button.state {
            Released => match button.time.released < 2 {
                true => match button.time.pressed > ms_to_ticks(hold_limit) {
                    true => None,
                    false => match key1 {
                        Key::KeyCode(kc) => Some(Key::KeyCode(kc)),
                        Key::Function(f) => Some(Key::Function(f)),
                        Key::PassThrough(go_down) => State::get_key(position, layer - go_down, button),
                        Key::LayerMo(l) => Some(Key::LayerMo(l)),
                        _ => None
                    },
                },
                false => None,
            },
            Pressed => {
                match button.time.pressed > ms_to_ticks(hold_limit) {
                    true => match key2 {
                        Key::KeyCode(kc) => Some(Key::KeyCode(kc)),
                        Key::Function(f) => Some(Key::Function(f)),
                        Key::PassThrough(go_down) => State::get_key(position, layer - go_down, button),
                        Key::LayerMo(l) => Some(Key::LayerMo(l)),
                        _ => None
                    },
                    false => None,
                }
            }
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
