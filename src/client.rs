use super::Device;
use super::UserAgent;
use super::OS;
use super::{Deserialize, Serialize};

/// Houses the `Device`, `OS`, and `UserAgent` structs, which each get parsed
/// out from a user agent string by a `UserAgentParser`.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Client<'a> {
    pub device: Device<'a>,
    pub os: OS<'a>,
    pub user_agent: UserAgent<'a>,
}
