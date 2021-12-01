use std::borrow::Cow;

use derive_more::{Display, From};
use regex::Regex;

use super::{
    client::Client,
    device::Device,
    file::{DeviceParserEntry, OSParserEntry, RegexFile, UserAgentParserEntry},
    os::OS,
    parser::{
        device::Error as DeviceError, os::Error as OSError,
        user_agent::Error as UserAgentError,
    },
    user_agent::UserAgent,
    Parser, SubParser,
};

mod device;
mod os;
mod user_agent;

#[derive(Debug, Display, From)]
pub enum Error {
    IO(std::io::Error),
    Yaml(serde_yaml::Error),
    Device(DeviceError),
    OS(OSError),
    UserAgent(UserAgentError),
}

/// Handles the actual parsing of a user agent string by delegating to
/// the respective `SubParser`
#[derive(Debug)]
pub struct UserAgentParser {
    device_matchers: Vec<device::Matcher>,
    os_matchers: Vec<os::Matcher>,
    user_agent_matchers: Vec<user_agent::Matcher>,
}

impl Parser for UserAgentParser {
    /// Returns the full `Client` info when given a user agent string
    fn parse(&self, user_agent: &str) -> Client {
        Client {
            device: self.parse_device(user_agent),
            os: self.parse_os(user_agent),
            user_agent: self.parse_user_agent(user_agent),
        }
    }

    /// Returns just the `Device` info when given a user agent string
    fn parse_device(&self, user_agent: &str) -> Device {
        self.device_matchers
            .iter()
            .find_map(|matcher| matcher.try_parse(user_agent))
            .unwrap_or_default()
    }

    /// Returns just the `OS` info when given a user agent string
    fn parse_os(&self, user_agent: &str) -> OS {
        self.os_matchers
            .iter()
            .find_map(|matcher| matcher.try_parse(user_agent))
            .unwrap_or_default()
    }

    /// Returns just the `UserAgent` info when given a user agent string
    fn parse_user_agent(&self, user_agent: &str) -> UserAgent {
        self.user_agent_matchers
            .iter()
            .find_map(|matcher| matcher.try_parse(user_agent))
            .unwrap_or_default()
    }
}

impl UserAgentParser {
    /// Attempts to construct a `UserAgentParser` from the path to a file
    pub fn from_yaml(path: &str) -> Result<UserAgentParser, Error> {
        let file = std::fs::File::open(path)?;
        UserAgentParser::from_file(file)
    }

    /// Attempts to construct a `UserAgentParser` from a slice of raw bytes. The
    /// intention with providing this function is to allow using the
    /// `include_bytes!` macro to compile the `regexes.yaml` file into the
    /// the library by a consuming application.
    ///
    /// ```rust
    /// # use uaparser::*;
    /// let regexes = include_bytes!("../../src/core/regexes.yaml");
    /// let parser = UserAgentParser::from_bytes(regexes);
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Result<UserAgentParser, Error> {
        let regex_file: RegexFile = serde_yaml::from_slice(bytes)?;
        UserAgentParser::try_from(regex_file)
    }

    /// Attempts to construct a `UserAgentParser` from a reference to an open
    /// `File`. This `File` should be a the `regexes.yaml` depended on by
    /// all the various implementations of the UA Parser library.
    pub fn from_file(file: std::fs::File) -> Result<UserAgentParser, Error> {
        let regex_file: RegexFile = serde_yaml::from_reader(file)?;
        UserAgentParser::try_from(regex_file)
    }

    pub fn try_from(regex_file: RegexFile) -> Result<UserAgentParser, Error> {
        let mut device_matchers = Vec::with_capacity(regex_file.device_parsers.len());
        let mut os_matchers = Vec::with_capacity(regex_file.os_parsers.len());
        let mut user_agent_matchers =
            Vec::with_capacity(regex_file.user_agent_parsers.len());

        for parser in regex_file.device_parsers {
            device_matchers.push(device::Matcher::try_from(parser)?);
        }

        for parser in regex_file.os_parsers {
            os_matchers.push(os::Matcher::try_from(parser)?);
        }

        for parser in regex_file.user_agent_parsers {
            user_agent_matchers.push(user_agent::Matcher::try_from(parser)?);
        }

        Ok(UserAgentParser {
            device_matchers,
            os_matchers,
            user_agent_matchers,
        })
    }
}

pub(self) fn none_if_empty<T: AsRef<str>>(s: T) -> Option<T> {
    if !s.as_ref().is_empty() {
        Some(s)
    } else {
        None
    }
}

pub(self) fn replace(replacement: &str, captures: &regex::Captures) -> String {
    if replacement.contains('$') && captures.len() > 0 {
        let mut target = String::new();
        captures.expand(replacement, &mut target);
        target.trim().to_owned()
    } else {
        replacement.to_owned()
    }
}

lazy_static::lazy_static! {
    static ref INVALID_ESCAPES: Regex = Regex::new("\\\\([! /])").unwrap();
}

fn clean_escapes(pattern: &str) -> Cow<'_, str> {
    INVALID_ESCAPES.replace_all(pattern, "$1")
}
