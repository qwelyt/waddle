use arduino_hal::delay_ms;
use arduino_hal::hal::port::{Dynamic, PD1, PF4};
use arduino_hal::port::mode::{Floating, Input, Output, PullUp};
use arduino_hal::port::Pin;
use atmega_usbd::UsbBus;
use heapless::Vec;
use usb_device::device::{UsbDevice, UsbDeviceState};
use usbd_hid::descriptor::KeyboardReport;
use usbd_hid::hid_class::HIDClass;

use crate::event::Event;
use crate::keycode::k;
use crate::layout::{BUTTONS, COLS, Key, LAYERS, Layout, LAYOUT, LEDS, NUM_CHUNKS, ROWS};
use crate::position::position::Position;
use crate::scan::Scan;
use crate::state::State;

pub type RowPinType = Pin<Output>;
pub type ColPinType = Pin<Input<PullUp>>;

pub const DELAY_MS: u16 = 10;

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
    state: State,
    last_events: Vec<Event, BUTTONS>,
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
            state: State::new(),
            last_events: Vec::new(),
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
            state: State::new(),
            last_events: Vec::new(),
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
            let scan = self.scan();
            self.state.tick(&scan);
            let events = self.state.events();

            if !events.eq(&self.last_events) {
                self.set_leds();
                self.last_events = events.clone();
                let kr: KeyboardReport = self.create_report(events);
                self.hid_class.push_input(&kr);
            }

            // if !state.eq(&self.last_state) {
            //     // self.leds[0].set_low();
            //     // self.leds[1].set_high();
            //     let state = LAYOUT.apply_functions(&state);
            //     self.set_leds(&state);
            //     let kr: KeyboardReport = self.create_report(&state);
            //     self.hid_class.push_input(&kr);
            //     self.last_state = state;
            // } else {
            //     // self.leds[1].set_low();
            //     // self.leds[0].set_high();
            // }
            delay_ms(DELAY_MS);
        }
    }

    fn scan(&mut self) -> Scan {
        return match self.scan_type {
            ScanType::ROW2COL => {
                self.scan_row2col()
            }
            ScanType::COL2ROW => {
                self.scan_col2row()
            }
        }
    }

    fn scan_row2col(&mut self) -> Scan {
        let mut scan_state = Scan::new();
        for (r, row) in self.rows.iter_mut().enumerate() {
            Self::low(row);
            for (c, col) in self.cols.iter_mut().enumerate() {
                if Self::is_low(col) {
                    scan_state.set_pressed(r, c);
                }
            }
            Self::high(row);
        }
        scan_state
    }
    fn scan_col2row(&mut self) -> Scan {
        Scan::new()
    }

    fn set_leds(&mut self) {
        for (i, active) in self.state.led_state().iter().enumerate() {
            match *active {
                true => Self::low(self.leds.get_mut(i).unwrap()),
                false => Self::high(self.leds.get_mut(i).unwrap()),
            }
        }
    }

    fn create_report(&self, events: Vec<Event, BUTTONS>) -> KeyboardReport {
        if !events.is_empty() {
            let keycodes: Vec<u8, BUTTONS> = events.iter()
                .map(|e| match e {
                    Event::KeyCode(kc) => Some(*kc),
                    _ => None,
                })
                .filter(Option::is_some)
                .map(Option::unwrap)
                .collect();

            let mods: u8 = keycodes.iter()
                .map(|u| *u)
                .filter(k::is_mod)
                .map(k::to_mod_bitfield)
                .sum();

            let keys: Vec<u8, BUTTONS> = keycodes.iter()
                .map(|u| *u)
                .filter(|key| !k::is_mod(key))
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
