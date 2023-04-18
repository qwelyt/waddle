use arduino_hal::delay_ms;
use arduino_hal::hal::port::{Dynamic, PD1, PF4};
use arduino_hal::port::mode::{Floating, Input, Output, PullUp};
use arduino_hal::port::Pin;
use atmega_usbd::UsbBus;
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
                if report_buf[0] & 2 != 0 {} else {}
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

            let mods: u8 = state.pressed()
                .iter()
                .map(|p| LAYOUT.get_mod(layer, p))
                .filter(Option::is_some)
                .map(Option::unwrap)
                .sum();

            let keys: Vec<u8, BUTTONS> = state.pressed()
                .iter()
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
