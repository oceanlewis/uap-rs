use super::{client::*, device::*, file::*, os::*, user_agent::*, Parser, SubParser};
use serde_yaml;

mod device;
mod os;
mod user_agent;

#[derive(Debug)]
pub struct UserAgentParser {
    device_matchers: Vec<device::Matcher>,
    os_matchers: Vec<os::Matcher>,
    user_agent_matchers: Vec<user_agent::Matcher>,
}

impl Parser for UserAgentParser {
    fn parse(&self, user_agent: &str) -> Client {
        let device = self.parse_device(&user_agent);
        let os = self.parse_os(&user_agent);
        let user_agent = self.parse_user_agent(&user_agent);

        Client {
            device,
            os,
            user_agent,
        }
    }

    fn parse_device(&self, user_agent: &str) -> Device {
        self.device_matchers
            .iter()
            .filter_map(|matcher| matcher.try_parse(&user_agent))
            .collect::<Vec<Device>>()
            .first()
            .map(Device::to_owned)
            .unwrap_or_default()
    }

    fn parse_os(&self, user_agent: &str) -> OS {
        self.os_matchers
            .iter()
            .filter_map(|matcher| matcher.try_parse(&user_agent))
            .collect::<Vec<OS>>()
            .first()
            .map(OS::to_owned)
            .unwrap_or_default()
    }

    fn parse_user_agent(&self, user_agent: &str) -> UserAgent {
        self.user_agent_matchers
            .iter()
            .filter_map(|matcher| matcher.try_parse(&user_agent))
            .collect::<Vec<UserAgent>>()
            .first()
            .map(UserAgent::to_owned)
            .unwrap_or_default()
    }
}

impl UserAgentParser {
    pub fn from_yaml(path: &str) -> UserAgentParser {
        let file = std::fs::File::open(path).expect("File not found!");
        UserAgentParser::from_file(file)
    }

    pub fn from_file(file: std::fs::File) -> UserAgentParser {
        let regex_file: RegexFile = serde_yaml::from_reader(file).expect("Serde Error");
        UserAgentParser::from(regex_file)
    }
}

impl From<RegexFile> for UserAgentParser {
    fn from(regex_file: RegexFile) -> UserAgentParser {
        UserAgentParser {
            user_agent_matchers: regex_file
                .user_agent_parsers
                .into_iter()
                .map(user_agent::Matcher::from)
                .collect(),

            os_matchers: regex_file
                .os_parsers
                .into_iter()
                .map(os::Matcher::from)
                .collect(),

            device_matchers: regex_file
                .device_parsers
                .into_iter()
                .map(device::Matcher::from)
                .collect(),
        }
    }
}

pub(self) fn empty_string_is_none(s: &str) -> Option<String> {
    if !s.is_empty() {
        Some(s.to_string())
    } else {
        None
    }
}

pub(self) fn replace(replacement: &str, captures: &onig::Captures) -> String {
    if replacement.contains('$') && !captures.is_empty() {
        (1..=captures.len())
            .fold(replacement.to_owned(), |state: String, i: usize| {
                let group = captures.at(i).unwrap_or_default();
                state.replace(&format!("${}", i), &group)
            })
            .trim()
            .to_owned()
    } else {
        replacement.to_owned()
    }
}
