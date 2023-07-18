use std::borrow::Cow;

use derive_more::{Display, From};
use regex::Regex;

use super::{client::Client, file::RegexFile, Parser, SubParser};

mod device;
use super::{
    device::Device, file::DeviceParserEntry, parser::device::Error as DeviceError,
};

mod os;
use super::{file::OSParserEntry, os::OS, parser::os::Error as OSError};

mod user_agent;
use super::{
    file::UserAgentParserEntry, parser::user_agent::Error as UserAgentError,
    user_agent::UserAgent,
};

mod builder;
use self::builder::UserAgentParserBuilder;

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
    fn parse<'a>(&self, user_agent: &'a str) -> Client<'a> {
        Client {
            device: self.parse_device(user_agent),
            os: self.parse_os(user_agent),
            user_agent: self.parse_user_agent(user_agent),
        }
    }

    /// Returns just the `Device` info when given a user agent string
    fn parse_device<'a>(&self, user_agent: &'a str) -> Device<'a> {
        self.device_matchers
            .iter()
            .find_map(|matcher| matcher.try_parse(user_agent))
            .unwrap_or_default()
    }

    /// Returns just the `OS` info when given a user agent string
    fn parse_os<'a>(&self, user_agent: &'a str) -> OS<'a> {
        self.os_matchers
            .iter()
            .find_map(|matcher| matcher.try_parse(user_agent))
            .unwrap_or_default()
    }

    /// Returns just the `UserAgent` info when given a user agent string
    fn parse_user_agent<'a>(&self, user_agent: &'a str) -> UserAgent<'a> {
        self.user_agent_matchers
            .iter()
            .find_map(|matcher| matcher.try_parse(user_agent))
            .unwrap_or_default()
    }
}

impl UserAgentParser {
    pub fn builder() -> UserAgentParserBuilder {
        UserAgentParserBuilder::new()
    }

    /// Attempts to construct a `UserAgentParser` from the path to a file
    pub fn from_yaml(path: &str) -> Result<UserAgentParser, Error> {
        let file = std::fs::File::open(path)?;
        UserAgentParser::from_file(file)
    }

    fn _build_from_yaml(
        path: &str,
        builder: UserAgentParserBuilder,
    ) -> Result<UserAgentParser, Error> {
        let file = std::fs::File::open(path)?;
        Self::_build_from_file(file, builder)
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
        Self::try_from(regex_file)
    }

    fn _build_from_bytes(
        bytes: &[u8],
        builder: UserAgentParserBuilder,
    ) -> Result<UserAgentParser, Error> {
        let regex_file: RegexFile = serde_yaml::from_slice(bytes)?;
        Self::_try_from(
            regex_file,
            builder.device,
            builder.os,
            builder.user_agent,
            builder.unicode,
        )
    }

    /// Attempts to construct a `UserAgentParser` from a reference to an open
    /// `File`. This `File` should be a the `regexes.yaml` depended on by
    /// all the various implementations of the UA Parser library.
    pub fn from_file(file: std::fs::File) -> Result<UserAgentParser, Error> {
        let regex_file: RegexFile = serde_yaml::from_reader(file)?;
        Self::try_from(regex_file)
    }

    fn _build_from_file(
        file: std::fs::File,
        builder: UserAgentParserBuilder,
    ) -> Result<UserAgentParser, Error> {
        let regex_file: RegexFile = serde_yaml::from_reader(file)?;
        Self::_try_from(
            regex_file,
            builder.device,
            builder.os,
            builder.user_agent,
            builder.unicode,
        )
    }

    pub fn try_from(regex_file: RegexFile) -> Result<UserAgentParser, Error> {
        Self::_try_from(regex_file, true, true, true, true)
    }

    fn _try_from(
        regex_file: RegexFile,
        device: bool,
        os: bool,
        user_agent: bool,
        unicode: bool,
    ) -> Result<UserAgentParser, Error> {
        let device_matchers = if device {
            let mut matchers = Vec::with_capacity(regex_file.device_parsers.len());
            for parser in regex_file.device_parsers {
                matchers.push(device::Matcher::try_from(parser, unicode)?);
            }
            matchers
        } else {
            vec![]
        };

        let os_matchers = if os {
            let mut matchers = Vec::with_capacity(regex_file.os_parsers.len());
            for parser in regex_file.os_parsers {
                matchers.push(os::Matcher::try_from(parser, unicode)?);
            }
            matchers
        } else {
            vec![]
        };

        let user_agent_matchers = if user_agent {
            let mut matchers = Vec::with_capacity(regex_file.user_agent_parsers.len());
            for parser in regex_file.user_agent_parsers {
                matchers.push(user_agent::Matcher::try_from(parser, unicode)?);
            }
            matchers
        } else {
            vec![]
        };

        Ok(UserAgentParser {
            device_matchers,
            os_matchers,
            user_agent_matchers,
        })
    }
}

#[inline]
pub(self) fn none_if_empty<T: AsRef<str>>(s: T) -> Option<T> {
    if s.as_ref().is_empty() {
        None
    } else {
        Some(s)
    }
}

#[inline]
pub(self) fn has_group(replacement: &str) -> bool {
    replacement.contains('$')
}

#[inline]
pub(self) fn replace_cow<'a>(
    replacement: &str,
    replacement_has_group: bool,
    captures: &regex::bytes::Captures,
) -> Cow<'a, str> {
    if replacement_has_group && captures.len() > 0 {
        let mut target = vec![];
        let raw_replacement = replacement.as_bytes();
        captures.expand(raw_replacement, &mut target);
        std::str::from_utf8(&target)
            .map(|s| Cow::Owned(s.trim().to_owned()))
            // What is the behavior if we can't parse a string???
            .unwrap_or_else(|_| Cow::Owned(replacement.to_owned()))
    } else {
        Cow::Owned(replacement.to_owned())
    }
}

#[inline]
pub(self) fn match_to_str(m: regex::bytes::Match) -> Option<&str> {
    std::str::from_utf8(m.as_bytes()).ok()
}

lazy_static::lazy_static! {
    static ref INVALID_ESCAPES: Regex = Regex::new("\\\\([! /])").unwrap();
}

fn clean_escapes(pattern: &str) -> Cow<'_, str> {
    INVALID_ESCAPES.replace_all(pattern, "$1")
}
