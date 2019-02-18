use super::{Deserialize, Device, UserAgent, OS};

/// Houses the `Device`, `OS`, and `UserAgent` structs, which each get parsed
/// out from a user agent string by a `UserAgentParser`.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Client {
    pub device: Device,
    pub os: OS,
    pub user_agent: UserAgent,
}
