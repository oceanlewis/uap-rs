use std::borrow::Cow;

use super::{Deserialize, Serialize};

/// Describes the `Family` as well as the `Major`, `Minor`, `Patch`, and
/// `PatchMinor` versions of an `OS`
#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct OS<'a> {
    pub family: Cow<'a, str>,
    pub major: Option<Cow<'a, str>>,
    pub minor: Option<Cow<'a, str>>,
    pub patch: Option<Cow<'a, str>>,
    pub patch_minor: Option<Cow<'a, str>>,
}

impl<'a> Default for OS<'a> {
    fn default() -> Self {
        Self {
            family: Cow::Borrowed("Other"),
            major: None,
            minor: None,
            patch: None,
            patch_minor: None,
        }
    }
}
