#![allow(unused)]

use serde_derive::Deserialize;
use serde_yaml;

trait UserAgentParser {
    type Item;
    fn parse(&self, stringable: impl std::string::ToString) -> Self::Item;
}

struct Client {
    user_agent: UserAgent,
    os: OS,
    device: Device,
}

type UserAgentFamily = String;
type UserAgentMajor = String;
type UserAgentMinor = String;
type UserAgentPatch = String;

struct UserAgent {
    family: UserAgentFamily,
    major: Option<UserAgentMajor>,
    minor: Option<UserAgentMinor>,
    patch: Option<UserAgentPatch>,
}

type OSFamily = String;
type OSMajor = String;
type OSMinor = String;
type OSPatch = String;
type OSPatchMinor = String;

struct OS {
    family: OSFamily,
    major: Option<OSMajor>,
    minor: Option<OSMinor>,
    patch: Option<OSPatch>,
    patch_minor: Option<OSPatchMinor>,
}

type DeviceFamily = String;
type DeviceBrand = String;
type DeviceModel = String;

struct Device {
    family: DeviceFamily,
    brand: Option<DeviceBrand>,
    model: Option<DeviceModel>,
}

struct UserAgentMatcher {
    regex: regex::Regex,
    family_replacement: Option<String>,
    device_replacement: Option<String>,
}

impl From<UserAgentParserEntry> for UserAgentMatcher {
    fn from(entry: UserAgentParserEntry) -> UserAgentMatcher {
        let regex_builder = regex::RegexBuilder::new(&entry.regex);
        let regex = regex_builder.build();

        if regex.is_err() {
            println!("{:#?}", entry.regex);
        }

        UserAgentMatcher {
            regex: regex.expect("Regex failed to build"),
            family_replacement: entry.family_replacement,
            device_replacement: entry.device_replacement,
        }
    }
}

struct OSMatcher {
    regex: regex::Regex,
    os_replacement: Option<String>,
    os_v1_replacement: Option<String>,
    os_v2_replacement: Option<String>,
    os_v3_replacement: Option<String>,
}

impl From<OSParserEntry> for OSMatcher {
    fn from(entry: OSParserEntry) -> OSMatcher {
        let regex_builder = regex::RegexBuilder::new(&entry.regex);
        let regex = regex_builder.build();

        if regex.is_err() {
            println!("{:#?}", entry.regex);
        }

        OSMatcher {
            regex: regex.expect("Regex failed to build"),
            os_replacement: entry.os_replacement,
            os_v1_replacement: entry.os_v1_replacement,
            os_v2_replacement: entry.os_v2_replacement,
            os_v3_replacement: entry.os_v3_replacement,
        }
    }
}

struct DeviceMatcher {
    regex: regex::Regex,
    device_replacement: Option<String>,
    brand_replacement: Option<String>,
    model_replacement: Option<String>,
}

impl From<DeviceParserEntry> for DeviceMatcher {
    fn from(entry: DeviceParserEntry) -> DeviceMatcher {
        let case_insensitive = (entry.regex_flag == Some("i".to_string()));
        let mut regex_builder = regex::RegexBuilder::new(&entry.regex);
        regex_builder.case_insensitive(case_insensitive);

        let regex = regex_builder.build();

        if regex.is_err() {
            println!("{:#?}", entry.regex);
        }

        DeviceMatcher {
            regex: regex.expect("Regex failed to build"),
            device_replacement: entry.device_replacement,
            brand_replacement: entry.brand_replacement,
            model_replacement: entry.model_replacement,
        }
    }
}

struct Parser {
    user_agent_matchers: Vec<UserAgentMatcher>,
    os_matchers: Vec<OSMatcher>,
    device_matchers: Vec<DeviceMatcher>,
}

impl From<RegexFile> for Parser {
    fn from(regex_file: RegexFile) -> Parser {
        Parser {
            user_agent_matchers: regex_file
                .user_agent_parsers
                .into_iter()
                .map(UserAgentMatcher::from)
                .collect(),

            os_matchers: regex_file
                .os_parsers
                .into_iter()
                .map(OSMatcher::from)
                .collect(),

            device_matchers: regex_file
                .device_parsers
                .into_iter()
                .map(DeviceMatcher::from)
                .collect(),
        }
    }
}

impl Parser {
    pub fn from_yaml(path: &str) -> Parser {
        let mut file = std::fs::File::open(path).expect("File not found!");
        Parser::from_file(file)
    }

    pub fn from_file(file: std::fs::File) -> Parser {
        let mut regex_file: RegexFile = serde_yaml::from_reader(file).expect("Serde Error");

        let fix = |regex: &String| -> String {
            regex
                .replace(r"\/", r"/")
        };

        for parser in regex_file.device_parsers.iter_mut() {
            parser.regex = fix(&parser.regex);
        }

        for parser in regex_file.os_parsers.iter_mut() {
            parser.regex = fix(&parser.regex);
        }

        for parser in regex_file.user_agent_parsers.iter_mut() {
            parser.regex = fix(&parser.regex);
        }

        Parser::from(regex_file)
    }
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
