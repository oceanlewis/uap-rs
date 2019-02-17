use super::Deserialize;

pub type DeviceFamily = String;
pub type DeviceBrand = String;
pub type DeviceModel = String;

#[derive(Clone, Debug, Deserialize)]
pub struct Device {
  pub family: DeviceFamily,
  pub brand: Option<DeviceBrand>,
  pub model: Option<DeviceModel>,
}

impl Default for Device {
  fn default() -> Device {
    Device {
      family: "Other".to_string(),
      brand: None,
      model: None,
    }
  }
}
