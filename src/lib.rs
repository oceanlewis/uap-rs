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
//!
//! Alternatively you can use the `UserAgentParserBuilder` to create a parser:
//! ```rust
//! # use uaparser::*;
//! let ua_parser = UserAgentParser::builder().build_from_yaml("./src/core/regexes.yaml").expect("Parser creation failed");
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

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::module_name_repetitions)]

use serde_derive::{Deserialize, Serialize};

mod client;
mod device;
pub use device::Device;

mod os;
pub use os::OS;

mod user_agent;
pub use user_agent::UserAgent;

mod file;
mod parser;

pub use parser::{Error, UserAgentParser};

pub use client::Client;

pub trait Parser {
    fn parse<'a>(&self, user_agent: &'a str) -> Client<'a>;
    fn parse_device<'a>(&self, user_agent: &'a str) -> Device<'a>;
    fn parse_os<'a>(&self, user_agent: &'a str) -> OS<'a>;
    fn parse_user_agent<'a>(&self, user_agent: &'a str) -> UserAgent<'a>;
}

pub(crate) trait SubParser<'a> {
    type Item;
    fn try_parse(&self, text: &'a str) -> Option<Self::Item>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{borrow::Cow, fmt::Debug};

    #[test]
    fn parse_os_with_unicode() {
        let parser = UserAgentParser::builder()
            .with_unicode_support(true)
            .build_from_yaml("./src/core/regexes.yaml")
            .expect("Parser creation failed");
        do_parse_os_test_with_parser(&parser)
    }

    #[test]
    fn parse_os_without_unicode() {
        let parser = UserAgentParser::builder()
            .with_unicode_support(false)
            .build_from_yaml("./src/core/regexes.yaml")
            .expect("Parser creation failed");
        do_parse_os_test_with_parser(&parser)
    }

    fn do_parse_os_test_with_parser(parser: &UserAgentParser) {
        #[derive(Deserialize, Debug)]
        struct OSTestCases<'a> {
            test_cases: Vec<OSTestCase<'a>>,
        }

        #[derive(Deserialize, Debug)]
        struct OSTestCase<'a> {
            user_agent_string: Cow<'a, str>,
            family: Cow<'a, str>,
            major: Option<Cow<'a, str>>,
            minor: Option<Cow<'a, str>>,
            patch: Option<Cow<'a, str>>,
            patch_minor: Option<Cow<'a, str>>,
        }

        let test_os = std::fs::File::open("./src/core/tests/test_os.yaml")
            .expect("test_device.yaml failed to load");

        let additional_os_tests =
            std::fs::File::open("./src/core/test_resources/additional_os_tests.yaml")
                .expect("additional_os_tests.yaml failed to load");

        let test_cases: OSTestCases = serde_yaml::from_reader(test_os)
            .expect("Failed to deserialize device test cases");

        let additional_cases: OSTestCases = serde_yaml::from_reader(additional_os_tests)
            .expect("Failed to deserialize additional test cases");

        let mut total_passed = 0;
        let mut failed = Vec::new();

        for test_case in test_cases
            .test_cases
            .iter()
            .chain(additional_cases.test_cases.iter())
        {
            let os = parser.parse_os(&test_case.user_agent_string);

            if test_eq(&os, &test_case) {
                total_passed += 1;
            } else {
                failed.push((os.clone(), test_case));
            }
        }

        println!(
            "parse_os - Test Summary: {} out of {} test cases passed",
            total_passed,
            total_passed + failed.len()
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
    fn parse_device_with_unicode() {
        let parser = UserAgentParser::builder()
            .with_unicode_support(true)
            .build_from_yaml("./src/core/regexes.yaml")
            .expect("Parser creation failed");
        do_parse_device_test_with_parser(&parser)
    }

    #[test]
    fn parse_device_without_unicode() {
        let parser = UserAgentParser::builder()
            .with_unicode_support(false)
            .build_from_yaml("./src/core/regexes.yaml")
            .expect("Parser creation failed");
        do_parse_device_test_with_parser(&parser)
    }

    fn do_parse_device_test_with_parser(parser: &UserAgentParser) {
        #[derive(Deserialize, Debug)]
        struct DeviceTestCases<'a> {
            test_cases: Vec<DeviceTestCase<'a>>,
        }

        #[derive(Deserialize, Debug)]
        struct DeviceTestCase<'a> {
            user_agent_string: Cow<'a, str>,
            family: Cow<'a, str>,
            brand: Option<Cow<'a, str>>,
            model: Option<Cow<'a, str>>,
        }

        let file = std::fs::File::open("./src/core/tests/test_device.yaml")
            .expect("test_device.yaml failed to load");

        let test_cases: DeviceTestCases = serde_yaml::from_reader(file)
            .expect("Failed to deserialize device test cases");

        let mut total_passed = 0;
        let mut failed = Vec::new();

        for test_case in &test_cases.test_cases {
            let dev = parser.parse_device(&test_case.user_agent_string);

            if test_eq(&dev, &test_case) {
                total_passed += 1;
            } else {
                failed.push((dev, test_case));
            }
        }

        println!(
            "parse_device - Test Summary: {} out of {} test cases passed",
            total_passed,
            total_passed + failed.len()
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
    fn parse_user_agent_with_unicode() {
        let parser = UserAgentParser::builder()
            .with_unicode_support(true)
            .build_from_yaml("./src/core/regexes.yaml")
            .expect("Parser creation failed");
        do_parse_user_agent_test_with_parser(&parser)
    }

    #[test]
    fn parse_user_agent_without_unicode() {
        let parser = UserAgentParser::builder()
            .with_unicode_support(false)
            .build_from_yaml("./src/core/regexes.yaml")
            .expect("Parser creation failed");
        do_parse_user_agent_test_with_parser(&parser)
    }
    fn do_parse_user_agent_test_with_parser(parser: &UserAgentParser) {
        #[derive(Deserialize, Debug)]
        struct UserAgentTestCases<'a> {
            test_cases: Vec<UserAgentTestCase<'a>>,
        }

        #[derive(Deserialize, Debug)]
        struct UserAgentTestCase<'a> {
            user_agent_string: Cow<'a, str>,
            family: Cow<'a, str>,
            major: Option<Cow<'a, str>>,
            minor: Option<Cow<'a, str>>,
            patch: Option<Cow<'a, str>>,
        }

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

        let mut total_passed = 0;
        let mut failed = Vec::new();

        for test_case in test_cases
            .test_cases
            .iter()
            .chain(firefox_user_agent_test_cases.test_cases.iter())
            .chain(opera_mini_test_cases.test_cases.iter())
        {
            let ua = parser.parse_user_agent(&test_case.user_agent_string);

            if test_eq(&ua, &test_case) {
                total_passed += 1;
            } else {
                failed.push((ua, test_case));
            }
        }

        println!(
            "parse_user_agent - Test Summary: {} out of {} test cases passed",
            total_passed,
            total_passed + failed.len()
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
