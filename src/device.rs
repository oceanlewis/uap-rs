use super::{Deserialize, Serialize};

pub type Family = String;
pub type Brand = String;
pub type Model = String;

/// Describes the `Family`, `Brand` and `Model` of a `Device`
#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
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
