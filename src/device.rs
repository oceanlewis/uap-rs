use super::Deserialize;

pub type DeviceFamily = String;
pub type DeviceBrand = String;
pub type DeviceModel = String;

#[derive(Debug, Deserialize)]
pub struct Device {
    family: DeviceFamily,
    brand: Option<DeviceBrand>,
    model: Option<DeviceModel>,
}
