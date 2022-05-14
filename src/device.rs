use super::{Deserialize, Serialize};
use std::borrow::Cow;

/// Describes the `Family`, `Brand` and `Model` of a `Device`
#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Device<'a> {
    pub family: Cow<'a, str>,
    pub brand: Option<Cow<'a, str>>,
    pub model: Option<Cow<'a, str>>,
}

impl<'a> Default for Device<'a> {
    fn default() -> Self {
        Self {
            family: Cow::Borrowed("Other"),
            brand: None,
            model: None,
        }
    }
}
