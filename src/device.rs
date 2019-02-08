pub type DeviceFamily = String;
pub type DeviceBrand = String;
pub type DeviceModel = String;

pub struct Device {
    family: DeviceFamily,
    brand: Option<DeviceBrand>,
    model: Option<DeviceModel>,
}
