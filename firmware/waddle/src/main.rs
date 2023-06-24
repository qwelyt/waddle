#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(unused)]

use core::fmt::{Result, Write};
use core::panic::PanicInfo;

use arduino_hal::{delay_ms, entry, Peripherals, pins, Pins, port::{
    mode::{
        Input,
        Output,
        PullUp,
    },
    Pin,
}};
use arduino_hal::hal::port::Dynamic;
use arduino_hal::port::mode::{AnyInput, Floating};
use arduino_hal::port::PinMode;
use atmega_usbd::UsbBus;
use avr_device::{asm::sleep, interrupt};
use heapless::Vec;
use usb_device::{
    class_prelude::UsbBusAllocator,
    device::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
};
use usbd_hid::{
    descriptor::{KeyboardReport, SerializedDescriptor},
    hid_class::HIDClass,
};
use usbd_serial::SerialPort;

use layout::{COLS, Layout, LEDS, ROWS};

use crate::keyboard::{Keyboard, ScanType};
use crate::layout::LAYOUT;

mod layout;
mod state;
mod keycode;
mod keyboard;
mod position;
mod macros;
mod scan;

/// Wrapper around a usb-cdc SerialPort
/// to be able to use the `write!()` macro with it
pub struct DebugPort<'a>(SerialPort<'a, UsbBus>);

impl<'a> Write for DebugPort<'a> {
    fn write_str(&mut self, s: &str) -> Result {
        let _ = self.0.write(s.as_bytes());
        Ok(())
    }
}


#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins: Pins = pins!(peripherals);
    let pll = peripherals.PLL;
    let usb = peripherals.USB_DEVICE;

    // Configure pll
    // Set to 8MHz
    pll.pllcsr.write(|w| w.pindiv().set_bit());
    // Run 64MHz timers
    pll.pllfrq.write(|w| w.pdiv().mhz96().plltm().factor_15().pllusb().set_bit());
    // And enable
    pll.pllcsr.modify(|_, w| w.plle().set_bit());
    // Wait until the bit is set
    while pll.pllcsr.read().plock().bit_is_clear() {}

    unsafe {
        let usb_bus = unsafe {
            static mut UB: Option<UsbBusAllocator<UsbBus>> = None;
            &*UB.insert(UsbBus::new(usb))
        };
        USB_BUS = Some(usb_bus);

        // Set up the USB Communications Class Device driver for debugging
        let mut debug_port = DebugPort(SerialPort::new(USB_BUS.unwrap()));

        init_keyboard(pins);

        write!(debug_port, "hello").unwrap();
        interrupt::enable()
    };


    loop {
        sleep();
    }
}

static mut USB_BUS: Option<&UsbBusAllocator<UsbBus>> = None;
static mut KEYBOARD: Option<Keyboard> = None;

fn init_keyboard(pins: Pins) {
    unsafe {
        let hid_class = HIDClass::new(USB_BUS.unwrap(), KeyboardReport::desc(), 1);
        let usb_device = UsbDeviceBuilder::new(USB_BUS.unwrap(), UsbVidPid(0x16c0, 0x27db))
            .manufacturer("qwelyt")
            .product("waddle")
            .device_class(3) // HID
            .device_sub_class(1) // Keyboard
            .build();
        let rows = vec![
            pins.a2.into_output_high().downgrade(),
            pins.a3.into_output_high().downgrade(),
            pins.d2.into_output_high().downgrade(),
            pins.d3.into_output_high().downgrade(),
        ];
        let cols = vec![
            pins.a1.into_pull_up_input().downgrade(),
            pins.a0.into_pull_up_input().downgrade(),
            pins.d15.into_pull_up_input().downgrade(),
            pins.d16.into_pull_up_input().downgrade(),
            pins.d14.into_pull_up_input().downgrade(),
            pins.d10.into_pull_up_input().downgrade(),
            pins.d9.into_pull_up_input().downgrade(),
            pins.d8.into_pull_up_input().downgrade(),
            pins.d7.into_pull_up_input().downgrade(),
            pins.d6.into_pull_up_input().downgrade(),
            pins.d5.into_pull_up_input().downgrade(),
            pins.d4.into_pull_up_input().downgrade(),
        ];

        let mut leds = vec![
            pins.led_rx.into_output().downgrade(),
            pins.rx.into_output().downgrade(),
            pins.tx.into_output().downgrade(),
        ];
        leds.iter_mut().map(|p| p.set_high());

        KEYBOARD = Some(Keyboard::row2col(
            usb_device,
            hid_class,
            rows,
            cols,
            leds,
        ))
    }
}

#[interrupt(atmega32u4)]
fn USB_GEN() {
    unsafe { poll_usb() };
}

#[interrupt(atmega32u4)]
fn USB_COM() {
    unsafe { poll_usb() };
}

unsafe fn poll_usb() {
    let ctx = unsafe { KEYBOARD.as_mut().unwrap() };
    ctx.poll();
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let peripherals = unsafe { Peripherals::steal() };
    let pins = pins!(peripherals);
    let mut rx = pins.led_rx.into_output();
    let mut tx = pins.led_tx.into_output();
    loop {
        for _ in 0..2 {
            rx.set_high();
            tx.set_high();
            delay_ms(300);
            rx.set_low();
            tx.set_low();
            delay_ms(300);
        }
        for _ in 0..2 {
            rx.set_high();
            tx.set_high();
            delay_ms(100);
            rx.set_low();
            tx.set_low();
            delay_ms(100);
        }
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub unsafe extern "C" fn rust_eh_personality() -> () {}
