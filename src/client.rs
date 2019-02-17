use super::{Deserialize, Device, UserAgent, OS};

#[derive(Debug, Deserialize)]
pub struct Client {
  pub device: Device,
  pub os: OS,
  pub user_agent: UserAgent,
}
