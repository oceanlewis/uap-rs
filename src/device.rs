use super::Deserialize;

pub type Family = String;
pub type Brand = String;
pub type Model = String;

/// Describes the `Family`, `Brand` and `Model` of a `Device`
#[derive(Clone, Debug, Deserialize)]
pub struct Device {
    pub family: Family,
    pub brand: Option<Brand>,
    pub model: Option<Model>,
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

impl std::cmp::PartialEq for Device {
    fn eq(&self, rhs: &Device) -> bool {
        self.family == rhs.family && self.brand == rhs.brand && self.model == rhs.model
    }
}
