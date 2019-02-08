#![allow(unused)]

use serde_derive::Deserialize;
use serde_yaml;

mod client;
mod device;
mod file;
mod os;
mod parser;
mod user_agent;

pub use client::Client;
pub use device::Device;
pub use file::*;
pub use os::OS;
pub use parser::UserAgentParser;
pub use user_agent::UserAgent;

pub trait Parser {
    type Item;
    fn parse(&self, stringable: impl std::string::ToString) -> Self::Item;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_creation() {
        let parser = UserAgentParser::from_yaml("./src/core/regexes.yaml");
    }
}
