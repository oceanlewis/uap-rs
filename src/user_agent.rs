use super::Deserialize;

pub type UserAgentFamily = String;
pub type UserAgentMajor = String;
pub type UserAgentMinor = String;
pub type UserAgentPatch = String;

#[derive(Clone, Debug, Deserialize)]
pub struct UserAgent {
  pub family: UserAgentFamily,
  pub major: Option<UserAgentMajor>,
  pub minor: Option<UserAgentMinor>,
  pub patch: Option<UserAgentPatch>,
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

impl std::cmp::Eq for UserAgent {}

impl std::cmp::PartialEq for UserAgent {
  fn eq(&self, rhs: &UserAgent) -> bool {
    self.family == rhs.family
      && self.major == rhs.major
      && self.minor == rhs.minor
      && self.patch == rhs.patch
  }
}

impl std::cmp::PartialOrd for UserAgent {
  fn partial_cmp(&self, rhs: &UserAgent) -> std::option::Option<std::cmp::Ordering> {
    if self.major > rhs.major {
      return Some(std::cmp::Ordering::Greater);
    }

    if self.minor > rhs.minor {
      return Some(std::cmp::Ordering::Greater);
    }

    if self.patch > rhs.patch {
      return Some(std::cmp::Ordering::Greater);
    }

    None
  }
}

impl std::cmp::Ord for UserAgent {
  fn cmp(&self, other: &UserAgent) -> std::cmp::Ordering {
    self
      .partial_cmp(other)
      .unwrap_or(std::cmp::Ordering::Less)
  }
}
