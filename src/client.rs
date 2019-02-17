use super::{Deserialize, Device, UserAgent, OS};

/// Houses the `Device`, `OS`, and `UserAgent` structs, which each get parsed
/// out from a user agent string by a `UserAgentParser`.
#[derive(Debug, Deserialize)]
pub struct Client {
    pub device: Device,
    pub os: OS,
    pub user_agent: UserAgent,
}

impl std::cmp::PartialEq for Client {
    fn eq(&self, rhs: &Client) -> bool {
        self.device == rhs.device
            && self.os == rhs.os
            && self.user_agent == rhs.user_agent
    }
}
