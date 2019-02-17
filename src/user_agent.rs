use super::Deserialize;

pub type Family = String;
pub type Major = String;
pub type Minor = String;
pub type Patch = String;

/// Describes the `Family` as well as the `Major`, `Minor`, and `Patch` versions
/// of a `UserAgent` client
#[derive(Clone, Debug, Deserialize)]
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

impl std::cmp::PartialEq for UserAgent {
    fn eq(&self, rhs: &UserAgent) -> bool {
        self.family == rhs.family
            && self.major == rhs.major
            && self.minor == rhs.minor
            && self.patch == rhs.patch
    }
}
