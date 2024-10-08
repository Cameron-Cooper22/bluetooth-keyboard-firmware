use nrf_softdevice::ble::Uuid;

// https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Assigned_Numbers/out/en/Assigned_Numbers.pdf?v=1728314270309
pub(crate) const BLE_HID_SERVICE_UUID: Uuid = Uuid::new_16(0x2a22);

pub(crate) enum BleSpecification {
    DeviceInformation = 0x180a,
    /**
     * Don't know if I will use this yet, will have to for AP Research but I can take a pause on
     * it.
     *
     * BatteryService = 0x180f,
     */
}

pub(crate) enum BleCharacteristics {
    BatteryLevel,
    ModelNumber,
    SerialNumber,
    FirmwareRevision,
    HardwareRevision,
    SoftwareRevision,
    ManufacturerName,
    PnpID,
    HidInfo,
    ReportMap,
    HidControlPoint,
    HidReport,
    ProtocolMode,
}

pub(crate) enum BleDescriptor {
    ReportReference,
}

impl BleDescriptor {
    pub(crate) fn uuid(self) -> Uuid {
        Uuid::new_16(self as u16)
    }
}

impl BleSpecification {
    pub(crate) fn uuid(self) -> Uuid {
        Uuid::new_16(self as u16)
    }
}

impl BleCharacteristics {
    pub(crate) fn uuid(self) -> Uuid {
        Uuid::new_16(self as u16)
    }
}
