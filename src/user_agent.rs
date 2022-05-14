use std::borrow::Cow;

use super::{Deserialize, Serialize};

/// Describes the `Family` as well as the `Major`, `Minor`, and `Patch` versions
/// of a `UserAgent` client
#[derive(Clone, Debug, Deserialize, Serialize, Eq, Hash, PartialEq)]
pub struct UserAgent<'a> {
    pub family: Cow<'a, str>,
    pub major: Option<Cow<'a, str>>,
    pub minor: Option<Cow<'a, str>>,
    pub patch: Option<Cow<'a, str>>,
}

impl<'a> Default for UserAgent<'a> {
    fn default() -> Self {
        Self {
            family: Cow::Borrowed("Other"),
            major: None,
            minor: None,
            patch: None,
        }
    }
}
