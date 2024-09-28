use core::marker::PhantomData;
use usb_device::{
    class_prelude::{ ControlIn, ControlOut, DescriptorWriter, EndpointAddress, EndpointIn, InterfaceNumber, StringIndex, UsbBus, UsbBusAllocator, UsbClass },
    control::{ Request, RequestType },
    LangID, Result
};

#[rustfmt::skip]
const KD: &[u8] = &[
];

const USB_CLASS_HID: u8 = 0x03;
