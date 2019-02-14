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
    fn parse(&self, user_agent: &str) -> Option<Client>;
    fn parse_device(&self, user_agent: &str) -> Option<Device>;
    fn parse_os(&self, user_agent: &str) -> Option<OS>;
    fn parse_user_agent(&self, user_agent: &str) -> Option<UserAgent>;
}

pub trait SubParser {
    type Item;
    fn try_parse(&self, text: &str) -> Option<Self::Item>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn parse_device() {
    //     #[derive(Deserialize, Debug)]
    //     struct DeviceTestCases {
    //         test_cases: Vec<DeviceTestCase>,
    //     }

    //     #[derive(Deserialize, Debug)]
    //     struct DeviceTestCase {
    //         user_agent_string: String,
    //         family: String,
    //         brand: String,
    //         model: String,
    //     }

    //     let parser = UserAgentParser::from_yaml("./src/core/regexes.yaml");
    //     let mut file = std::fs::File::open("./src/core/tests/test_device.yaml")
    //         .expect("test_device.yaml failed to load");
    //     let test_cases: DeviceTestCases =
    //         serde_yaml::from_reader(&mut file).expect("Failed to deserialize device test cases");

    //     for test_case in test_cases.test_cases.into_iter() {
    //         let ua = parser.parse_device(&test_case.user_agent_string);
    //     }
    // }

    #[test]
    fn parse_user_agent() {
        #[derive(Deserialize, Debug)]
        struct UserAgentTestCases {
            test_cases: Vec<UserAgentTestCase>,
        }

        #[derive(Deserialize, Debug)]
        struct UserAgentTestCase {
            user_agent_string: String,
            family: String,
            major: Option<String>,
            minor: Option<String>,
            patch: Option<String>,
        }

        let parser = UserAgentParser::from_yaml("./src/core/regexes.yaml");
        let mut file = std::fs::File::open("./src/core/tests/test_ua.yaml")
            .expect("test_device.yaml failed to load");
        let test_cases: UserAgentTestCases =
            serde_yaml::from_reader(&mut file).expect("Failed to deserialize device test cases");

        for test_case in test_cases.test_cases.into_iter() {
            let ua = parser.parse_user_agent(&test_case.user_agent_string);
            if ua.is_none() {
                println!("{:#?}", test_case);
            }
            let ua = ua.unwrap();

            assert_same(ua, test_case);
        }

        fn assert_same(ua: UserAgent, test_case: UserAgentTestCase) {
            if ua.family != test_case.family
                || ua.major != test_case.major
                || ua.minor != test_case.minor
                || ua.patch != test_case.patch
            {
                println!("Parsed:\n{:#?}\nExpected:\n{:#?}", ua, test_case);
            }

            assert_eq!(ua.family, test_case.family);
            assert_eq!(ua.major, test_case.major);
            assert_eq!(ua.minor, test_case.minor);
            assert_eq!(ua.patch, test_case.patch);
        }
    }
}
