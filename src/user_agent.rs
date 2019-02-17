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
