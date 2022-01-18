use super::{Deserialize, Device, Serialize, UserAgent, OS};

/// Houses the `Device`, `OS`, and `UserAgent` structs, which each get parsed
/// out from a user agent string by a `UserAgentParser`.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Client {
    pub device: Device,
    pub os: OS,
    pub user_agent: UserAgent,
}
