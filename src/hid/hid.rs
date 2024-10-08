use core::marker::PhantomData;
use usb_device::{
    class_prelude::{ ControlIn, ControlOut, DescriptorWriter, EndpointAddress, EndpointIn, InterfaceNumber, StringIndex, UsbBus, UsbBusAllocator, UsbClass },
    control::{ Request, RequestType },
    LangID, Result
};

const USB_CLASS_HID: u8 = 0x03;

const DESCRIPTOR_LEN_BYTES: [u8; 2] = (BOOT_KD.len() as u16).to_le_bytes(); 

#[allow(dead_code)]
#[rustfmt::skip]
const BOOT_KD: &[u8] = &[
    0x05, 0x01,     // Usage Page (Generic Desktop)
    0x09, 0x06,     // Usage ID (Keyboard)
    0xa1, 0x01,     // Collection (Application)

    // Mod
    0x85, 0x00,     //      Report ID (0)
    0x05, 0x07,     //      Usage (Keypad)
    0x19, 0xe0,     //      Usage Minimum (224)
    0x29, 0xe7,     //      Usage Maximum (231)
    0x15, 0x00,     //      Logical Minimum
    0x25, 0x01,     //      Logical Maximum
    0x95, 0x08,     //      Report Count (8)
    0x75, 0x01,     //      Report Size (1)
    0x81, 0x02,     //      Input (Data,Var,Abs)

    // Reserved Vyte for HID spec. This can change to 
    // LEDs later but I will hold off on it.
    0x95, 0x01,     //      Report Count (1)
    0x75, 0x08,     //      Report Size (8)
    0x81, 0x01,     //      Input (Const, Var, Abs)

    //Keycodes  
    0x05, 0x07,     //      Usage (Keypad)
    0x19, 0x00,     //      Usage Minimum (0)
    0x29, 0xff,     //      Usage Maximum ()
    0x15, 0x00,     //      Logical Minumum
    0x25, 0xff,     //      Logical Maximum
    0x85, 0x06,     //      Report Count (6)
    0x75, 0x08,     //      Report Size (8)
    0x81, 0x00,     //      Input (Data, Array)
];

const FIRST_BIT     : u8 = 0x01;
const LAST_BIT      : u8 = 0xa4;    // The very last possible keycode that IS NOT a modifier key.
const NUM_BITS      : u8 = LAST_BIT - FIRST_BIT + 1;
const NUM_BYTES     : u8 = ((NUM_BITS + 8 - 1)/ 8);
const NUM_PADDING   : u8 = NUM_BITS % 8;

// Contains 0xa4 len bitfield, 1 byte for modifiers, 1 byte for consumer control, 6 boot desc
// bytes. The 6 boot dest bytes will be an attempt to get this to work, because it most likely will
// not.
pub const REP_SIZE : u8 = NUM_BYTES + 8;

/**
 * This is basically an attempt to have both boot and NRKO working. I am HIGHLY unsure if this will
 * work, and will need to double check with a virtual keyboard possibly.
 */
#[rustfmt::skip]
const NRKO_KD: &[u8] = &[
  0x05, 0x01,   // Usage Page (Generic Desktop)
  0x09, 0x06,   // Usage ID (Keyboard)
  0xa1, 0x01,   // Collection (Application)
  
  // Modifier Byte
  0x05, 0x07,   //    Usage (Keypad)
  0x19, 0xe0,   //    Usage Minimum (224)
  0x29, 0xe7,   //    Usage Maximum (231)
  0x15, 0x00,   //    Logical Minimum (0)
  0x25, 0x01,   //    Logical Maximum (1)
  0x95, 0x08,   //    Report Count (8)
  0x75, 0x01,   //    Report Size (1)
  0x81, 0x02,   //    Input (Data, Var, Abs)
  
  // Media Byte. Reserved in Boot
  0x05, 0x0c,   //    Usage Page (Consumer)
  0x15, 0x00,   //    Logical Minimum (0)
  0x25, 0x01,   //    Logical Maximum (1)
  0x75, 0x01,   //    Report Size (1)
  0x95, 0x08,   //    Report Count (8)
  0x09, 0xb5,   //    Usage (Scan Next Track)
  0x09, 0xb6,   //    Usage (Scan Next Track)
  0x09, 0xcd,   //    Usage (Play/Pause)
  0x09, 0xe9,   //    Usage (VolInc)
  0x09, 0xea,   //    Usage (VolDec)
  0x09, 0xe2,   //    Usage (Mute)
  0x09, 0xd1,   //    Usage (Start/Stop Game Recording)
  0x09, 0xd3,   //    Usage (Game Capture)
  0x81, 0x02,   //    Input (Data, Var, Abs)
  
  // Boot Desc Bytes??       TODO: Check if this even works lmao
  
  0x95, 0x06,   //    Report Count (6)
  0x75, 0x08,   //    Report Size (8)
  0x81, 0x03,   //    Input (Constant)

  0x05, 0x07,   //    Usage Page (Keyboard)
  0x19, FIRST_BIT, //    Usage Minimum (FIRST_BIT)
  0x29, LAST_BIT, //    Usage Maximum (LAST_BIT)
  0x15, 0,  //    Logical Minimum (0)
  0x25, 1,  //    Logical Maximum (1)
  0x95, NUM_BITS, //    Report Count (NUM_BITS)
  0x75, 0x01,   //    Report Size (1)
  0x81, 0x02,   //    Input (Data, Var, Abs)
  //
  0x95, NUM_PADDING, //    Report Count (NUM_PADDING)
  0x75, 0x01,   //    Report Size (1)
  0x81, 0x03,   //    Input (Constant)

  0xc0,         // End Collection
];

#[repr(u8)]
#[derive(Clone, Copy)]
pub(crate) enum VidSource {
    BluetoothSIG = 1,
    UsbIF = 2,
}

/// PnP ID characteristic is a set of values used to craete an unique device ID.
/// These values are used to identify all devices of a given type/model/version using numbers.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub(crate) struct PnPID {
    pub(crate) vid_source: VidSource,
    pub(crate) vendor_id: u16,
    pub(crate) product_id: u16,
    pub(crate) product_version: u16,
}

#[derive(Debug, Default, defmt::Format)]
pub(crate) struct DeviceInformation {
    pub(crate) manufacturer_name: Option<&'static str>,
    pub(crate) model_number: Option<&'static str>,
    pub(crate) serial_number: Option<&'static str>,
    pub(crate) hw_rev: Option<&'static str>,
    pub(crate) fw_rev: Option<&'static str>,
    pub(crate) sw_rev: Option<&'static str>,
}

pub struct Reporter {
    report : [u8; REP_SIZE as usize],
}

impl Reporter {
    pub const fn new() -> Self {
        Reporter {
            report: [0; REP_SIZE as usize]
        }
    }
}
