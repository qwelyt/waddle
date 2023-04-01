use arduino_hal::hal::port::Dynamic;
use arduino_hal::port::mode::{Input, Output, PullUp};
use arduino_hal::port::Pin;
use atmega_usbd::UsbBus;
use hash32::{BuildHasherDefault, FnvHasher, Hasher};
use heapless::{FnvIndexSet, IndexSet, Vec};
use usb_device::device::UsbDevice;
use usbd_hid::descriptor::KeyboardReport;
use usbd_hid::hid_class::HIDClass;

use crate::layout::{COLS, LEDS, ROWS};
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
    current_state: State,
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
    ) -> Self {
        // Put everything low to save ourselves from bad things.
        rows.iter_mut().map(|p| p.set_low());
        cols.iter_mut().map(|p| p.set_low());
        leds.iter_mut().map(|p| p.set_low());

        // let mut row_pins: [EitherPin; ROWS] = [EitherPin::None; ROWS];
        // let mut col_pins: [EitherPin; COLS] = [EitherPin::None; COLS];
        let mut row_pins: Vec<EitherPin, ROWS> = Vec::new();
        let mut col_pins: Vec<EitherPin, COLS> = Vec::new();
        match scan_type {
            ScanType::ROW2COL => {
                for (i, pin) in rows.into_iter().enumerate() {
                    row_pins[i] = EitherPin::Output(pin.into_output_high());
                }
                for (i, pin) in cols.into_iter().enumerate() {
                    col_pins[i] = EitherPin::Input(pin.into_pull_up_input());
                }
            }
            ScanType::COL2ROW => {
                for (i, pin) in rows.into_iter().enumerate() {
                    row_pins[i] = EitherPin::Input(pin.into_pull_up_input());
                }
                for (i, pin) in cols.into_iter().enumerate() {
                    col_pins[i] = EitherPin::Output(pin.into_output_high())
                }
            }
        };


        Self {
            usb_device,
            hid_class,
            scan_type,
            rows: row_pins,//.try_into().unwrap(),
            cols: col_pins,//.try_into().unwrap(),
            // rows: demo(row_pins),
            // cols: demo(col_pins),
            leds,
            current_state: State::empty(),
            last_state: State::empty(),
        }
    }
    pub fn poll(&mut self) {
        // let pin = self.leds.first().unwrap().downgrade();
        if !&self.current_state.eq(&self.last_state) {
            self.current_state = self.scan();
            let report = Keyboard::create_report(&self.current_state);
            self.hid_class.push_input(&report).ok();
        }
    }

    fn scan(&mut self) -> State {
        match self.scan_type {
            ScanType::ROW2COL => self.row2col(),
            ScanType::COL2ROW => self.col2row(),
        };
        State::empty()
    }


    fn create_report(x: &State) -> KeyboardReport {
        // Check functions before building keys
        // Be sure to reset layer before running layer-editing functions!

        let report = KeyboardReport {
            modifier: 0,
            reserved: 0,
            leds: 0,
            keycodes: [0; 6],
        };
        report
    }
    fn row2col(&mut self) -> IndexSet<Position, BuildHasherDefault<FnvHasher>, 64> {
        let mut pressed = FnvIndexSet::<Position, 64>::new();
        for (r, row) in self.rows.iter_mut().enumerate() {
            Self::low(row);
            for (c, col) in self.cols.iter_mut().enumerate() {
                if Self::is_low(col) {
                    pressed.insert(Position::new(
                        u8::try_from(r).unwrap(),
                        u8::try_from(c).unwrap(),
                    ));
                }
            }
            Self::high(row);
        }
        pressed
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
    fn col2row(&self) -> IndexSet<Position, BuildHasherDefault<FnvHasher>, 64> {
        FnvIndexSet::<Position, 64>::new()
    }
}