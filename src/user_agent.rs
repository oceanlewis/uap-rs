use super::Deserialize;

pub type UserAgentFamily = String;
pub type UserAgentMajor = String;
pub type UserAgentMinor = String;
pub type UserAgentPatch = String;

#[derive(Debug, Deserialize)]
pub struct UserAgent {
  pub family: UserAgentFamily,
  pub major: Option<UserAgentMajor>,
  pub minor: Option<UserAgentMinor>,
  pub patch: Option<UserAgentPatch>,
}
