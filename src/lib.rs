//! This crate is an implementation of a User Agent Parser, similar to those
//! found as part of the [UA-Parser Community](https://github.com/ua-parser). It tries to remain as
//! consistent with the other implementations as possible while remaining simple
//! and legible.
//!
//! ```rust
//! # use uaparser::*;
//! let ua_parser = UserAgentParser::from_yaml("./src/core/regexes.yaml").expect("Parser creation failed");
//! let user_agent_string =
//!     String::from("Mozilla/5.0 (X11; Linux x86_64; rv:2.0b8pre) Gecko/20101031 Firefox-4.0/4.0b8pre");
//! let client = ua_parser.parse(&user_agent_string);
//!
//! let device = ua_parser.parse_device(&user_agent_string);
//! let os = ua_parser.parse_os(&user_agent_string);
//! let user_agent = ua_parser.parse_user_agent(&user_agent_string);
//!
//! assert_eq!(client.device, device);
//! assert_eq!(client.os, os);
//! assert_eq!(client.user_agent, user_agent);
//! ```

use serde_derive::{Deserialize, Serialize};

mod client;
mod device;
mod file;
mod os;
mod parser;
mod user_agent;

pub use parser::{Error, UserAgentParser};

pub use client::Client;
pub use device::Device;
pub use os::OS;
pub use user_agent::UserAgent;

pub trait Parser {
    fn parse(&self, user_agent: &str) -> Client;
    fn parse_device(&self, user_agent: &str) -> Device;
    fn parse_os(&self, user_agent: &str) -> OS;
    fn parse_user_agent(&self, user_agent: &str) -> UserAgent;
}

pub(crate) trait SubParser {
    type Item;
    fn try_parse(&self, text: &str) -> Option<Self::Item>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;

    #[test]
    fn parse_os() {
        #[derive(Deserialize, Debug)]
        struct OSTestCases {
            test_cases: Vec<OSTestCase>,
        }

        #[derive(Deserialize, Debug)]
        struct OSTestCase {
            user_agent_string: String,
            family: String,
            major: Option<String>,
            minor: Option<String>,
            patch: Option<String>,
            patch_minor: Option<String>,
        }

        let parser = UserAgentParser::from_yaml("./src/core/regexes.yaml")
            .expect("Parser creation failed");

        let test_os = std::fs::File::open("./src/core/tests/test_os.yaml")
            .expect("test_device.yaml failed to load");

        let additional_os_tests =
            std::fs::File::open("./src/core/test_resources/additional_os_tests.yaml")
                .expect("additional_os_tests.yaml failed to load");

        let test_cases: OSTestCases = serde_yaml::from_reader(test_os)
            .expect("Failed to deserialize device test cases");

        let additional_cases: OSTestCases = serde_yaml::from_reader(additional_os_tests)
            .expect("Failed to deserialize additional test cases");

        let mut passed = Vec::new();
        let mut failed = Vec::new();

        for test_case in test_cases
            .test_cases
            .into_iter()
            .chain(additional_cases.test_cases.into_iter())
        {
            let os = parser.parse_os(&test_case.user_agent_string);

            if test_eq(&os, &test_case) {
                passed.push((os, test_case));
            } else {
                failed.push((os, test_case));
            }
        }

        println!(
            "parse_os - Test Summary: {} out of {} test cases passed",
            passed.len(),
            passed.len() + failed.len()
        );

        if !failed.is_empty() {
            for fail in failed.iter() {
                print_failure(&fail.0, &fail.1);
            }
        }

        assert!(failed.is_empty());

        fn test_eq(os: &OS, test_case: &OSTestCase) -> bool {
            os.family == test_case.family
                && os.major == test_case.major
                && os.minor == test_case.minor
                && os.patch == test_case.patch
                && os.patch_minor == test_case.patch_minor
        }
    }

    #[test]
    fn parse_device() {
        #[derive(Deserialize, Debug)]
        struct DeviceTestCases {
            test_cases: Vec<DeviceTestCase>,
        }

        #[derive(Deserialize, Debug)]
        struct DeviceTestCase {
            user_agent_string: String,
            family: String,
            brand: Option<String>,
            model: Option<String>,
        }

        let parser = UserAgentParser::from_yaml("./src/core/regexes.yaml")
            .expect("Parser creation failed");

        let file = std::fs::File::open("./src/core/tests/test_device.yaml")
            .expect("test_device.yaml failed to load");

        let test_cases: DeviceTestCases = serde_yaml::from_reader(file)
            .expect("Failed to deserialize device test cases");

        let mut passed = Vec::new();
        let mut failed = Vec::new();

        for test_case in test_cases.test_cases.into_iter() {
            let dev = parser.parse_device(&test_case.user_agent_string);

            if test_eq(&dev, &test_case) {
                passed.push((dev, test_case));
            } else {
                failed.push((dev, test_case));
            }
        }

        println!(
            "parse_device - Test Summary: {} out of {} test cases passed",
            passed.len(),
            passed.len() + failed.len()
        );

        if !failed.is_empty() {
            for fail in failed.iter() {
                print_failure(&fail.0, &fail.1);
            }
        }

        assert!(failed.is_empty());

        fn test_eq(dev: &Device, test_case: &DeviceTestCase) -> bool {
            dev.family == test_case.family
                && dev.brand == test_case.brand
                && dev.model == test_case.model
        }
    }

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

        let parser = UserAgentParser::from_yaml("./src/core/regexes.yaml")
            .expect("Parser creation failed");

        let test_ua = std::fs::File::open("./src/core/tests/test_ua.yaml")
            .expect("test_device.yaml failed to load");

        let firefox_user_agent_strings = std::fs::File::open(
            "./src/core/test_resources/firefox_user_agent_strings.yaml",
        )
        .expect("firefox_user_agent_strings.yaml failed to load");

        let opera_mini_user_agent_strings = std::fs::File::open(
            "./src/core/test_resources/opera_mini_user_agent_strings.yaml",
        )
        .expect("opera_mini_user_agent_strings.yaml failed to open");

        let test_cases: UserAgentTestCases = serde_yaml::from_reader(test_ua)
            .expect("Failed to deserialize device test cases");

        let firefox_user_agent_test_cases: UserAgentTestCases =
            serde_yaml::from_reader(firefox_user_agent_strings)
                .expect("Failed deserialize firefox test cases");

        let opera_mini_test_cases: UserAgentTestCases =
            serde_yaml::from_reader(opera_mini_user_agent_strings)
                .expect("Failed to deserialized opera mini test cases");

        let mut passed = Vec::new();
        let mut failed = Vec::new();

        for test_case in test_cases
            .test_cases
            .into_iter()
            .chain(firefox_user_agent_test_cases.test_cases.into_iter())
            .chain(opera_mini_test_cases.test_cases.into_iter())
        {
            let ua = parser.parse_user_agent(&test_case.user_agent_string);

            if test_eq(&ua, &test_case) {
                passed.push((ua, test_case));
            } else {
                failed.push((ua, test_case));
            }
        }

        println!(
            "parse_user_agent - Test Summary: {} out of {} test cases passed",
            passed.len(),
            passed.len() + failed.len()
        );

        if !failed.is_empty() {
            for fail in failed.iter() {
                print_failure(&fail.0, &fail.1);
            }
        }

        assert!(failed.is_empty());

        fn test_eq(ua: &UserAgent, test_case: &UserAgentTestCase) -> bool {
            ua.family == test_case.family
                && ua.major == test_case.major
                && ua.minor == test_case.minor
                && ua.patch == test_case.patch
        }
    }

    fn print_failure<T: Debug, F: Debug>(got: &T, expected: &F) {
        println!(
            r" --- Failed Test Case ----
Expected {:?}
Got {:?}
",
            expected, got
        );
    }
}
