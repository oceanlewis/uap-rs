use super::Deserialize;

pub type UserAgentFamily = String;
pub type UserAgentMajor = String;
pub type UserAgentMinor = String;
pub type UserAgentPatch = String;

#[derive(Debug, Deserialize)]
pub struct UserAgent {
    family: UserAgentFamily,
    major: Option<UserAgentMajor>,
    minor: Option<UserAgentMinor>,
    patch: Option<UserAgentPatch>,
}
