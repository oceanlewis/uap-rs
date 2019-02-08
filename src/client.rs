use super::{Deserialize, Device, UserAgent, OS};

#[derive(Debug, Deserialize)]
pub struct Client {
  user_agent: UserAgent,
  os: OS,
  device: Device,
}
