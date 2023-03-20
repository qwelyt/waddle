use arduino_hal::port::mode::{Input, Output, PullUp};
use arduino_hal::port::Pin;
use atmega_usbd::UsbBus;
use usb_device::device::UsbDevice;
use usbd_hid::descriptor::KeyboardReport;
use usbd_hid::hid_class::HIDClass;

use crate::layout::{COLS, LEDS, ROWS};
use crate::state::State;

pub type RowPinType = Pin<Output>;
pub type ColPinType = Pin<Input<PullUp>>;

pub struct Keyboard {
    usb_device: UsbDevice<'static, UsbBus>,
    hid_class: HIDClass<'static, UsbBus>,
    rows: [RowPinType; ROWS],
    cols: [ColPinType; COLS],
    leds: [Pin<Output>; LEDS],
    current_state: State,
    last_state: State,
}

impl Keyboard {
    pub fn new(
        usb_device: UsbDevice<'static, UsbBus>,
        hid_class: HIDClass<'static, UsbBus>,
        rows: [RowPinType; ROWS],
        cols: [ColPinType; COLS],
        leds: [Pin<Output>; LEDS],
    ) -> Self {
        Self {
            usb_device,
            hid_class,
            rows,
            cols,
            leds,
            current_state: State::new(),
            last_state: State::new(),
        }
    }
    pub fn poll(&mut self) {
        if !&self.current_state.eq(&self.last_state) {
            self.current_state = self.last_state.clone();
            let report = self.crete_report(&self.current_state);
            self.hid_class.push_input(&report).ok();
        }
    }


    fn crete_report(self, x: &State) -> KeyboardReport {
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
}
