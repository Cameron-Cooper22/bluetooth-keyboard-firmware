/**
 * Advertising in BlueTooth LE is the process of broadcasting the information from the bluetooth
 * device to a different host device through a 2.4 GHz band with 40 different channels. This method
 * is still fairly 'unsafe', but I am deadass just making a keyboard that is only going to stay in
 * my house. If someone somehow figures out how to capture the keyboard inputs, good for them at
 * that point, they deserve it.
 */
use nrf_softdevice::ble::advertisement_builder::{
    AdvertisementDataType, Flag, LegacyAdvertisementBuilder, LegacyAdvertisementPayload, ServiceList, ServiceUuid16
};

pub(crate) fn create_ad(keyboard_id: &str) -> LegacyAdvertisementPayload {
    LegacyAdvertisementBuilder::new()
        .flags(&[Flag::GeneralDiscovery, Flag::LE_Only])
        .services_16(ServiceList::Incomplete, &[ServiceUuid16::BATTERY, ServiceUuid16::HUMAN_INTERFACE_DEVICE])
        .full_name(keyboard_id)
        .raw(AdvertisementDataType, &[0xc1, 0x03])
        .build()
}

pub(crate) static SCAN_DATA: LegacyAdvertisementPayload = LegacyAdvertisementBuilder::new()
    .services_16(
        ServiceList::Complete, 
        &[ServiceUuid16::DEVICE_INFORMATION, ServiceUuid16::BATTERY, ServiceUuid16::HUMAN_INTERFACE_DEVICE]
        )
    .build();
