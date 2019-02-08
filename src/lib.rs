#![allow(unused)]

use serde_derive::Deserialize;
use serde_yaml;

mod client;
mod device;
mod os;
mod parser;
mod user_agent;

pub use client::Client;
pub use device::Device;
pub use os::OS;
pub use user_agent::UserAgent;
pub use parser::Parser;

pub trait UserAgentParser {
    type Item;
    fn parse(&self, stringable: impl std::string::ToString) -> Self::Item;
}

impl UserAgentParser for Parser {
    type Item = Client;

    fn parse(&self, agent: impl std::string::ToString) -> Self::Item {
        unimplemented!()
    }
}

#[derive(Debug, Deserialize)]
struct RegexFile {
    user_agent_parsers: Vec<UserAgentParserEntry>,
    os_parsers: Vec<OSParserEntry>,
    device_parsers: Vec<DeviceParserEntry>,
}

#[derive(Debug, Deserialize)]
struct UserAgentParserEntry {
    regex: String,
    family_replacement: Option<String>,
    device_replacement: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OSParserEntry {
    regex: String,
    os_replacement: Option<String>,
    os_v1_replacement: Option<String>,
    os_v2_replacement: Option<String>,
    os_v3_replacement: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DeviceParserEntry {
    regex_flag: Option<String>,
    regex: String,
    device_replacement: Option<String>,
    brand_replacement: Option<String>,
    model_replacement: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_creation() {
        let parser = Parser::from_yaml("./src/core/regexes.yaml");
    }
}
