use embassy_nrf::{
    gpio::{Input, Output},
    saadc::Saadc
};
use nrf52832_hal::gpio::{Input, Output};

pub struct BatteryConfig<'a> {
    pub charge_state_pin: Option<Input<'a>>,
    pub charge_led_pin: Option<Output<'a>>,
    pub charge_state_low: bool,
    pub charge_led_low: bool,
    pub saadc: Option<Saadc<'a, 1>>,
}
