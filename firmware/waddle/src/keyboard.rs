use arduino_hal::hal::port::Dynamic;
use arduino_hal::port::mode::{Input, Output, PullUp};
use arduino_hal::port::Pin;
use atmega_usbd::UsbBus;
use heapless::Vec;
use usb_device::device::UsbDevice;
use usbd_hid::descriptor::KeyboardReport;
use usbd_hid::hid_class::HIDClass;

use crate::layout::{BUTTONS, COLS, Key, LAYERS, Layout, LEDS, NUM_CHUNKS, ROWS};
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
    leds: Vec<Pin<Output>, LEDS>,
    layout: Layout,
    last_state: State,
}

impl Keyboard {
    pub fn new(
        usb_device: UsbDevice<'static, UsbBus>,
        hid_class: HIDClass<'static, UsbBus>,
        scan_type: ScanType,
        mut rows: Vec<Pin<Output>, ROWS>,
        mut cols: Vec<Pin<Output>, COLS>,
        mut leds: Vec<Pin<Output>, LEDS>,
        layout: Layout,
    ) -> Self {
        // Put everything low to save ourselves from bad things.
        // rows.iter_mut().for_each(|p| p.set_low());
        // cols.iter_mut().for_each(|p| p.set_low());
        // leds.iter_mut().for_each(|p| p.set_low());

        let mut row_pins: Vec<EitherPin, ROWS> = Vec::new();
        let mut col_pins: Vec<EitherPin, COLS> = Vec::new();
        // match scan_type {
        //     ScanType::ROW2COL => {
        //         for (i, pin) in rows.into_iter().enumerate() {
        //             row_pins.insert(i, EitherPin::Output(pin.into_output_high())).ok();
        //         }
        //         for (i, pin) in cols.into_iter().enumerate() {
        //             col_pins.insert(i, EitherPin::Input(pin.into_pull_up_input())).ok();
        //         }
        //     }
        //     ScanType::COL2ROW => {
        //         for (i, pin) in rows.into_iter().enumerate() {
        //             row_pins.insert(i, EitherPin::Input(pin.into_pull_up_input())).ok();
        //         }
        //         for (i, pin) in cols.into_iter().enumerate() {
        //             col_pins.insert(i, EitherPin::Output(pin.into_output_high())).ok();
        //         }
        //     }
        // };


        Self {
            usb_device,
            hid_class,
            scan_type,
            rows: row_pins,
            cols: col_pins,
            leds,
            layout,
            last_state: State::empty(),
        }
    }
    pub fn poll(&mut self) {
        if self.usb_device.poll(&mut [&mut self.hid_class]) {
            let mut report_buf = [0u8; 1];

            if self.hid_class.pull_raw_output(&mut report_buf).is_ok() {
                if report_buf[0] & 2 != 0 {
                    self.leds.iter_mut().for_each(|p| p.set_high());
                } else {
                    self.leds.iter_mut().for_each(|p| p.set_low());
                }
            }
        }
        // let current_state = self.scan();
        // if !&self.current_state.eq(&self.last_state) {
        //     let reports = Self::create_report(&self.current_state, &self.layout);
        //     for report in reports.iter() {
        //         self.hid_class.push_input(report).ok();
        //     }
        //     self.last_state = self.current_state.clone();
        // }
    }

    fn scan(&mut self) -> State {
        match self.scan_type {
            ScanType::ROW2COL => self.row2col(),
            ScanType::COL2ROW => self.col2row(),
        }
    }


    fn create_report(state: &State, layout: &Layout) -> Vec<KeyboardReport, NUM_CHUNKS> {
        // Check functions before building keys
        // Be sure to reset layer before running layer-editing functions!
        let layer: u8 = state.pressed().iter().map(|p| layout.get_layer_mod(p)).sum();

        let mods: u8 = state.pressed()
            .iter()
            .map(|p| layout.get_mod(layer, p))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .sum();

        let key_codes: Vec<u8, BUTTONS> = state.pressed()
            .iter()
            .map(|p| layout.get_non_mod(layer, p))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();


        let chunks = Self::chunkate(key_codes);
        let reports: Vec<KeyboardReport, NUM_CHUNKS> = chunks.iter()
            .map(|chunk| KeyboardReport {
                modifier: mods,
                reserved: 0,
                leds: 0,
                keycodes: chunk.clone(),
            }).collect();

        reports
    }

    fn chunkate(v: Vec<u8, BUTTONS>) -> Vec<[u8; 6], NUM_CHUNKS> {
        let mut chunks: Vec<[u8; 6], NUM_CHUNKS> = Vec::new();
        let mut ar: [u8; 6] = [0; 6];
        for (i, kc) in v.iter().enumerate() {
            if i % 6 == 0 {
                chunks.push(ar.clone());
                ar = [0; 6];
            }
            ar[i] = *kc;
        }
        if ar.iter().map(|u| *u as usize).sum::<usize>() != 0 {
            chunks.push(ar.clone());
        }
        chunks
    }
    fn row2col(&mut self) -> State {
        let mut state = State::empty();
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

    fn is_low(col: &mut EitherPin) -> bool {
        match col {
            EitherPin::Input(p) => { p.is_low() }
            EitherPin::Output(_) => { panic!() }
            EitherPin::None => { panic!() }
        }
    }

    fn high(row: &mut EitherPin) {
        match row {
            EitherPin::Input(_) => {}
            EitherPin::Output(p) => { p.set_high(); }
            EitherPin::None => {}
        }
    }

    fn low(row: &mut EitherPin) {
        match row {
            EitherPin::Input(_) => {}
            EitherPin::Output(p) => { p.set_low(); }
            EitherPin::None => {}
        }
    }
    fn col2row(&self) -> State {
        State::empty()
    }
}
