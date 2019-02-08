use super::*;

mod device;
mod os;
mod user_agent;

pub struct Parser {
  user_agent_matchers: Vec<user_agent::Matcher>,
  os_matchers: Vec<os::Matcher>,
  device_matchers: Vec<device::Matcher>,
}

impl Parser {
  pub fn from_yaml(path: &str) -> Parser {
    let mut file = std::fs::File::open(path).expect("File not found!");
    Parser::from_file(file)
  }

  pub fn from_file(file: std::fs::File) -> Parser {
    let mut regex_file: RegexFile = serde_yaml::from_reader(file).expect("Serde Error");

    let fix = |regex: &String| -> String { regex.replace(r"\/", r"/") };

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

impl From<RegexFile> for Parser {
  fn from(regex_file: RegexFile) -> Parser {
    Parser {
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
