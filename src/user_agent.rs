use super::{Deserialize, Serialize};

pub type Family = String;
pub type Major = String;
pub type Minor = String;
pub type Patch = String;

/// Describes the `Family` as well as the `Major`, `Minor`, and `Patch` versions
/// of a `UserAgent` client
#[derive(Clone, Debug, Deserialize, Serialize, Eq, Hash, PartialEq)]
pub struct UserAgent {
    pub family: Family,
    pub major: Option<Major>,
    pub minor: Option<Minor>,
    pub patch: Option<Patch>,
}

impl Default for UserAgent {
    fn default() -> UserAgent {
        UserAgent {
            family: "Other".to_string(),
            major: None,
            minor: None,
            patch: None,
        }
    }
}
