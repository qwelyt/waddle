use usb_device::bus::{UsbBus, UsbBusAllocator};
use usb_device::prelude::*;
use usb_device::device::UsbDevice;

const REPORT_VID:u16 = 0x16c0;
const REPORT_PID:u16 = 0x27db;
const HID_REPORT_DESCRIPTOR: &[u8] = &[
//  Keyboard
0x05, 0x01,                    // USAGE_PAGE (Generic Desktop)
0x09, 0x06,                    // USAGE (Keyboard)
0xa1, 0x01,                    // COLLECTION (Application)
REPORT_VID, REPORT_PID,               //   REPORT_ID (2)
0x05, 0x07,                    //   USAGE_PAGE (Keyboard)

0x19, 0xe0,                    //   USAGE_MINIMUM (Keyboard LeftControl)
0x29, 0xe7,                    //   USAGE_MAXIMUM (Keyboard RightGUI)
0x15, 0x00,                    //   LOGICAL_MINIMUM (0)
0x25, 0x01,                    //   LOGICAL_MAXIMUM (1)
0x75, 0x01,                    //   REPORT_SIZE (1)

0x95, 0x08,                      //   REPORT_COUNT (8)
0x81, 0x02,                    //   INPUT (Data,Var,Abs)
0x95, 0x01,                    //   REPORT_COUNT (1)
0x75, 0x08,                    //   REPORT_SIZE (8)
0x81, 0x03,                    //   INPUT (Cnst,Var,Abs)

0x95, 0x06,                      //   REPORT_COUNT (6)
0x75, 0x08,                    //   REPORT_SIZE (8)
0x15, 0x00,                    //   LOGICAL_MINIMUM (0)
0x25, 0x65,                    //   LOGICAL_MAXIMUM (101)
0x05, 0x07,                    //   USAGE_PAGE (Keyboard)

0x19, 0x00,                      //   USAGE_MINIMUM (Reserved (no event indicated))
0x29, 0x65,                    //   USAGE_MAXIMUM (Keyboard Application)
0x81, 0x00,                    //   INPUT (Data,Ary,Abs)
0xc0,                          // END_COLLECTION
];

pub fn new<B>(bus: &UsbBusAllocator<B>) -> UsbDevice<B> where B: UsbBus{
    UsbDeviceBuilder::new(bus, UsbVidPid(REPORT_VID, REPORT_PID))
        .manufacturer("qwelyt")
        .product("waddle")
        .serial_number(env!("CARGO_PKG_VERSION"))
        .device_class()
        .build()
}