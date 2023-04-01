#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(unused)]

use core::alloc::Layout;
use core::panic::PanicInfo;

use arduino_hal::{delay_ms,
                  entry,
                  Peripherals,
                  pins,
                  port::{
                      mode::{
                          Input,
                          Output,
                          PullUp,
                      },
                      Pin,
                  },
};
use arduino_hal::hal::port::Dynamic;
use arduino_hal::port::mode::{AnyInput, Floating};
use arduino_hal::port::PinMode;
use atmega_usbd::UsbBus;
use avr_device::{asm::sleep, interrupt};
use hash32::{BuildHasherDefault, FnvHasher};
use heapless::{FnvIndexSet, IndexSet, Vec};
use usb_device::{
    class_prelude::UsbBusAllocator,
    device::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
};
use usbd_hid::{
    descriptor::{KeyboardReport, SerializedDescriptor},
    hid_class::HIDClass,
};

use layout::{COLS, LEDS, ROWS};

use crate::keyboard::{Keyboard, ScanType};

mod layout;
mod state;
mod keycode;
mod keyboard;
mod position;
mod macros;


#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = pins!(peripherals);
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

    let usb_bus = unsafe {
        static mut USB_BUS: Option<UsbBusAllocator<UsbBus>> = None;
        &*USB_BUS.insert(UsbBus::new(usb))
    };

    let hid_class = HIDClass::new(&usb_bus, KeyboardReport::desc(), 1);
    let usb_device = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27db))
        .manufacturer("qwelyt")
        .product("waddle")
        .device_class(3) // HID
        .device_sub_class(1) // Keyboard
        .build();

    let rows = vec![
        pins.a0.downgrade().into_output(),
        pins.a1.downgrade().into_output(),
        pins.a2.downgrade().into_output(),
        pins.a3.downgrade().into_output(),
    ];

    let cols = vec![
        pins.d2.downgrade().into_output(),
        pins.d3.downgrade().into_output(),
        pins.d4.downgrade().into_output(),
        pins.d5.downgrade().into_output(),
        pins.d6.downgrade().into_output(),
        pins.d7.downgrade().into_output(),
        pins.d8.downgrade().into_output(),
        pins.d9.downgrade().into_output(),
        pins.d10.downgrade().into_output(),
        pins.d16.downgrade().into_output(),
        pins.d14.downgrade().into_output(),
        pins.d15.downgrade().into_output(),
    ];


    let leds = vec![
        pins.rx.downgrade().into_output(),
        pins.tx.downgrade().into_output(),
        pins.led_tx.downgrade().into_output(),
    ];


    unsafe {
        KEYBOARD = Some(Keyboard::new(
            usb_device,
            hid_class,
            ScanType::ROW2COL,
            rows,
            cols,
            leds,
        )
        )
    }

    unsafe { interrupt::enable() };

    loop {
        sleep();
    }
}

static mut KEYBOARD: Option<Keyboard> = None;

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
            delay_ms(50);
            rx.set_low();
            tx.set_low();
            delay_ms(50);
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
