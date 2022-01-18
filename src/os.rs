use super::{Deserialize, Serialize};

pub type Family = String;
pub type Major = String;
pub type Minor = String;
pub type Patch = String;
pub type PatchMinor = String;

/// Describes the `Family` as well as the `Major`, `Minor`, `Patch`, and
/// `PatchMinor` versions of an `OS`
#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct OS {
    pub family: Family,
    pub major: Option<Major>,
    pub minor: Option<Minor>,
    pub patch: Option<Patch>,
    pub patch_minor: Option<PatchMinor>,
}

impl Default for OS {
    fn default() -> OS {
        OS {
            family: "Other".to_string(),
            major: None,
            minor: None,
            patch: None,
            patch_minor: None,
        }
    }
}
