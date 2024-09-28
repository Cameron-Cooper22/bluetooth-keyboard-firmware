#![no_std]
#![no_main]

use embedded_hal::digital::InputPin;
use nrf52832_hal as hal;
use hal::gpio::{p0::Parts as P0Parts, Floating, Input, Level, Output, Pin, PushPull};
use hal::pac::Peripherals;
use hal::lpcomp::*;
#[panic_handler]
fn panic() {
    loop {};
}

pub struct PinManager {
    pins: [Pin<Input<Floating>>; 31],
    col_pins: [Pin<Input<Floating>>; 15],
    row_pins: [Pin<Output<PushPull>>; 5],
    ref_pin: Pin<Input<Floating>>,
    in_pin: Pin<Input<Floating>>,

}

impl PinManager {
    pub fn new() -> Self {
        let peripherals = Peripherals::take();
        let p0_parts = P0Parts::new(peripherals.P0);
        return PinManager {
            /**
             * The pin configuration will likely change later. This will depend heavily on
             * how I move the chip around on the actual PCB. p0_00 and p0_01 MUST change as they
             * are used by 32 MHz crystal. Also, need to figure out if I can use the output GPIO
             * pins for LPCOMP input. If there is a easy way to do that without reconfiguring the
             * pins each time that would be sick.
             *
             * The row GPIO pins will be given an event where as the mark for time until sleep
             * reaches 0, they will be reconfigured for lpcomp. 
             */
            // TODO: change the p0_00, p0_01, etc to their actual values.
            // Also, need to go back and check for AIN if I want to use
            // LPCOMP. Which I do. Very much. Please. I want it.
            pins: [
                p0_parts.p0_00, // Reserved for 32 kHz crystal
                p0_parts.p0_01, // Reserved for 32 kHz crystal
                p0_parts.p0_02, // AIN0
                p0_parts.p0_03, // AIN1
                p0_parts.p0_04, // AIN2
                p0_parts.p0_05, // AIN3
                p0_parts.p0_06,
                p0_parts.p0_07,
                p0_parts.p0_08,
                p0_parts.p0_09,
                p0_parts.p0_10,
                p0_parts.p0_11,
                p0_parts.p0_12,
                p0_parts.p0_13,
                p0_parts.p0_14,
                p0_parts.p0_15,
                p0_parts.p0_16,
                p0_parts.p0_17,
                p0_parts.p0_18,
                p0_parts.p0_19,
                p0_parts.p0_20,
                p0_parts.p0_21,
                p0_parts.p0_22,
                p0_parts.p0_23,
                p0_parts.p0_24,
                p0_parts.p0_25,
                p0_parts.p0_26,
                p0_parts.p0_27,
                p0_parts.p0_28, // AIN4
                p0_parts.p0_29, // AIN5
                p0_parts.p0_30, // AIN6
                p0_parts.p0_31, // AIN7
            ],
            col_pins: [
                p0_parts.p0_03.into_floating_input(),
                p0_parts.p0_04.into_floating_input(),
                p0_parts.p0_05.into_floating_input(),
                p0_parts.p0_06.into_floating_input(),
                p0_parts.p0_07.into_floating_input(),
                p0_parts.p0_08.into_floating_input(),
                p0_parts.p0_09.into_floating_input(),
                p0_parts.p0_10.into_floating_input(),
                p0_parts.p0_11.into_floating_input(),
                p0_parts.p0_12.into_floating_input(),
                p0_parts.p0_13.into_floating_input(),
                p0_parts.p0_14.into_floating_input(),
                p0_parts.p0_15.into_floating_input(),
                p0_parts.p0_16.into_floating_input(),
                p0_parts.p0_17.into_floating_input(),
                
            ],
            row_pins: [
                p0_parts.p0_18.into_push_pull_output(Level::Low),
                p0_parts.p0_19.into_push_pull_output(Level::Low),
                p0_parts.p0_20.into_push_pull_output(Level::Low),
                p0_parts.p0_21.into_push_pull_output(Level::Low),
                p0_parts.p0_22.into_push_pull_output(Level::Low),
            ],
            ref_pin: p0_parts.p0_02.into_floating_input(),
            in_pin: p0_parts.p0_31.into_floating_input(),
        };
    }

    // True if high, False if low
    pub fn read_pin(&self, pin_number: usize) -> bool {
        match pin_number {
            0 ..= 31 => self.pins[pin_number].is_high(),
            _ => panic!(),
        }
    }

    pub fn write_gpio(&self, pin_number: usize, high: bool) {
        // Range of row pins is valid, everything else is not.
        match pin_number {
            17..=22 =>  true,
            _ => panic!(),
        };
        
        if high {
            self.row_pins[pin_number].set_high();
        } else {
            self.row_pins[pin_number].set_low();
        }
    }
}
