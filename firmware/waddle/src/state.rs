use core::intrinsics::offset;

use heapless::Vec;

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
                self.pressed[i] = self.pressed[i] + 1;
            } else {
                // It's either 0 as it was never pressed which means there is no change
                // Or it was pressed and we should check for how long it was pressed to check for
                // on-holds.
                self.released[i] = self.pressed[i];
            }
        }
    }


    pub fn events(&self) {
        let mut buttons: Vec<[Key; LAYERS], BUTTONS> = Vec::new();
        for i in 0..BUTTONS {
            let ticks = self.released[i];
            if ticks > 2 {
                // Check for on-holds. If ticks are below the on-hold limit then
                // send the non-hold key as an event.
                // If the ticks are *over* then we have already activated that key
                // and should do nothing
            }

            let ticks = self.pressed[i];
            if ticks > 2 {
                buttons.push(LAYOUT.get(i));
            }
        }
        // 1. Find which layer we are on
        // 2. Get all keys on that layer
        // 3. Add KeyCodes and Functions as events
        // 4. Check if on-holds. If they are above limit, get the key.
        //      If they are below, ignore.
    }
    fn ms_to_ticks(ms: u8) -> u8 {
        ms / DELAY_MS
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