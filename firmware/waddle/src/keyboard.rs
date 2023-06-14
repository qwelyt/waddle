use arduino_hal::delay_ms;
use arduino_hal::hal::port::{Dynamic, PD1, PF4};
use arduino_hal::port::mode::{Floating, Input, Output, PullUp};
use arduino_hal::port::Pin;
use atmega_usbd::UsbBus;
use avr_device::atmega32u4::Interrupt::TIMER0_COMPA;
use heapless::Vec;
use usb_device::device::{UsbDevice, UsbDeviceState};
use usbd_hid::descriptor::KeyboardReport;
use usbd_hid::hid_class::HIDClass;

use crate::layout::{BUTTONS, COLS, Key, LAYERS, Layout, LAYOUT, LEDS, NUM_CHUNKS, ROWS};
use crate::position::position::Position;
use crate::state::State;

pub type RowPinType = Pin<Output>;
pub type ColPinType = Pin<Input<PullUp>>;

pub enum ScanType {
    ROW2COL,
    COL2ROW,
}

enum EitherPin {
    Input(Pin<Input<PullUp>>),
    Output(Pin<Output>),
    None,
}


pub struct Keyboard {
    usb_device: UsbDevice<'static, UsbBus>,
    hid_class: HIDClass<'static, UsbBus>,
    scan_type: ScanType,
    rows: Vec<EitherPin, ROWS>,
    cols: Vec<EitherPin, COLS>,
    leds: Vec<EitherPin, LEDS>,
    last_state: State,
}

impl Keyboard {
    pub fn row2col(
        usb_device: UsbDevice<'static, UsbBus>,
        hid_class: HIDClass<'static, UsbBus>,
        mut rows: Vec<Pin<Output>, ROWS>,
        mut cols: Vec<Pin<Input<PullUp>>, COLS>,
        mut leds: Vec<Pin<Output>, LEDS>,
    ) -> Self {
        let mut row_pins: Vec<EitherPin, ROWS> = Vec::new();
        let mut col_pins: Vec<EitherPin, COLS> = Vec::new();
        let mut led_pins: Vec<EitherPin, LEDS> = Vec::new();

        for (i, pin) in cols.into_iter().enumerate() {
            col_pins.insert(i, EitherPin::Input(pin));
        }
        for (i, pin) in rows.into_iter().enumerate() {
            row_pins.insert(i, EitherPin::Output(pin));
        }
        for (i, pin) in leds.into_iter().enumerate() {
            led_pins.insert(i, EitherPin::Output(pin));
        }
        Self {
            usb_device,
            hid_class,
            scan_type: ScanType::ROW2COL,
            rows: row_pins,
            cols: col_pins,
            leds: led_pins,
            last_state: State::empty(),
        }
    }
    pub fn col2row(
        usb_device: UsbDevice<'static, UsbBus>,
        hid_class: HIDClass<'static, UsbBus>,
        mut rows: Vec<Pin<Input<PullUp>>, ROWS>,
        mut cols: Vec<Pin<Output>, COLS>,
        mut leds: Vec<Pin<Output>, LEDS>,
    ) -> Self {
        let mut row_pins: Vec<EitherPin, ROWS> = Vec::new();
        let mut col_pins: Vec<EitherPin, COLS> = Vec::new();
        let mut led_pins: Vec<EitherPin, LEDS> = Vec::new();

        for (i, pin) in cols.into_iter().enumerate() {
            col_pins.insert(i, EitherPin::Output(pin));
        }
        for (i, pin) in rows.into_iter().enumerate() {
            row_pins.insert(i, EitherPin::Input(pin));
        }
        for (i, pin) in leds.into_iter().enumerate() {
            led_pins.insert(i, EitherPin::Output(pin));
        }
        Self {
            usb_device,
            hid_class,
            scan_type: ScanType::COL2ROW,
            rows: row_pins,
            cols: col_pins,
            leds: led_pins,
            last_state: State::empty(),
        }
    }
    pub fn poll(&mut self) {
        if self.usb_device.poll(&mut [&mut self.hid_class]) {
            let mut report_buf = [0u8; 1];
            if self.hid_class.pull_raw_output(&mut report_buf).is_ok() {
                // Bit | Led
                // 0   | Num lock
                // 1   | Caps lock
                // 2   | Scroll lock
                // 3   | Composition Mode
                // 4   | Kana Mode
                if report_buf[0] & 2 != 0 {
                    Self::low(self.leds.get_mut(1).unwrap());
                } else {
                    Self::high(self.leds.get_mut(1).unwrap());
                }
            }
        }
        if self.usb_device.state() == UsbDeviceState::Configured {
            let state = self.debounced_scan();
            if !state.eq(&self.last_state) {
                // self.leds[0].set_low();
                // self.leds[1].set_high();
                let state = LAYOUT.apply_functions(&state);
                self.set_leds(&state);
                let kr: KeyboardReport = self.create_report(&state);
                self.hid_class.push_input(&kr);
                self.last_state = state;
            } else {
                // self.leds[1].set_low();
                // self.leds[0].set_high();
            }
        }
    }

    fn debounced_scan(&mut self) -> State {
        let s1 = self.scan();
        delay_ms(20);
        let s2 = self.scan();
        State::intersect(s1, s2)
    }
    fn scan(&mut self) -> State {
        match self.scan_type {
            ScanType::ROW2COL => {
                return self.scan_row2col();
            }
            ScanType::COL2ROW => {
                return self.scan_col2row();
            }
        }
    }

    fn scan_row2col(&mut self) -> State {
        let mut state = self.last_state.clean();
        for (r, row) in self.rows.iter_mut().enumerate() {
            Self::low(row);
            for (c, col) in self.cols.iter_mut().enumerate() {
                if Self::is_low(col) {
                    state.set_pressed(r, c);
                }
            }
            Self::high(row);
        }
        state
    }
    fn scan_col2row(&mut self) -> State {
        State::empty()
    }

    fn set_leds(&mut self, state: &State) {
        for (i, active) in state.led_state().iter().enumerate() {
            match *active {
                true => Self::low(self.leds.get_mut(i).unwrap()),
                false => Self::high(self.leds.get_mut(i).unwrap()),
            }
        }
    }

    fn create_report(&self, state: &State) -> KeyboardReport {
        if !state.pressed().is_empty() {
            let layer = state.pressed()
                .iter()
                .map(|p| LAYOUT.get_layer_mod(p))
                .sum();

            let keys: Vec<Key, BUTTONS> = state.pressed()
                .iter()
                .map(|p| LAYOUT.get_key(layer, p))
                .collect();

            // 1: Find combos
            // 2: Filter out other keys taken by a combo (so we don't activate that ones primary)
            // 3: Evaluate combos
            // 4: Find mods
            // 5: Find keycodes

            // On holds?
            // How to deal with those?
            // Need a timer to see for how long something has been pressed, and if we have a key
            // that has an on hold we can't send a state with that key until release or timer "runs out".
            // So we need to change `state.pressed` here to be "activated`, and we activate keys on
            // release or timer run-out.
            // Another layer of indirection.
            // But if a key is pressed, and we don't have an on-hold, we still need to send that press.
            // Can't wait for release as that would mess up gaming (pressing W for forward etc.)
            // So first check pressed. Then check which has on-hold. Those that do not can be `active` directly
            // and operate like "normal". Those that have on-hold need to wait for release or timer.
            //
            // Timers should be stored in state. An Instant for when the key was first pressed. Then on
            // each check look if time > on_hold_time or if the key has been released. Of it is still
            // on hold then we get the key, else we get nothing and continue waiting. Option<Key> perhaps.
            // And the `Key` can be anything, layer, mod, function, anything. So we need to evaluate all holds
            // first to get the proper Key.
            // After on_hold we should either check combos or layers. I think layers first, then combos. That
            // way you can have the same keys for combos on different layers, meaning more keys. - Perhaps
            // that should be true for on_hold as well? hmm
            // Anyway, layer or on_hold first, then combos. Then we get all the other keys and check
            // which are Active. We get all the active_keys, and map those into either mods or printables.
            // And then we send the state.
            // Put all this in its own file so it is easier to reason about.


            // TODO: Change num allowed combos
            let combos: Vec<(Position, Key), 8> = state.pressed()
                .iter()
                .map(|p| LAYOUT.get_combo(layer, p))
                .filter(Option::is_some)
                .map(Option::unwrap)
                .collect();
            let combo_pos: Vec<Position, 8> = combos.iter().map(|c| c.0).collect();

            let pressed: Vec<Position, BUTTONS> = state.pressed()
                .iter()
                .filter(|p| combo_pos.contains(*p))
                .collect();

            let mods: u8 = pressed.iter()
                .map(|p| LAYOUT.get_mod(layer, p))
                .filter(Option::is_some)
                .map(Option::unwrap)
                .sum();

            let keys: Vec<u8, BUTTONS> = pressed.iter()
                .map(|p| LAYOUT.get_keycode(layer, p))
                .filter(Option::is_some)
                .map(Option::unwrap)
                .collect();


            let mut kc = [0; 6];
            for (i, k) in keys.iter().enumerate() {
                kc[i] = *k;
            }
            return KeyboardReport {
                modifier: mods,
                reserved: 0,
                leds: 0,
                keycodes: kc,
            };
        }
        KeyboardReport {
            modifier: 0,
            reserved: 0,
            leds: 0,
            keycodes: [0; 6],
        }
    }

    fn is_low(pin: &mut EitherPin) -> bool {
        match pin {
            EitherPin::Input(p) => { p.is_low() }
            EitherPin::Output(_) => { panic!() }
            EitherPin::None => { panic!() }
        }
    }
    fn is_high(pin: &mut EitherPin) -> bool {
        match pin {
            EitherPin::Input(p) => { p.is_high() }
            EitherPin::Output(_) => { panic!() }
            EitherPin::None => { panic!() }
        }
    }

    fn high(pin: &mut EitherPin) {
        match pin {
            EitherPin::Input(_) => {}
            EitherPin::Output(p) => { p.set_high(); }
            EitherPin::None => {}
        }
    }

    fn low(pin: &mut EitherPin) {
        match pin {
            EitherPin::Input(_) => {}
            EitherPin::Output(p) => { p.set_low(); }
            EitherPin::None => {}
        }
    }
}
