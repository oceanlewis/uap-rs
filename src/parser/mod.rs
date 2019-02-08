use super::{client::*, device::*, file::*, os::*, UserAgentParser};
use serde_yaml;

mod device;
mod os;
mod user_agent;

pub struct Parser {
  user_agent_matchers: Vec<user_agent::Matcher>,
  os_matchers: Vec<os::Matcher>,
  device_matchers: Vec<device::Matcher>,
}

fn split_regexes(regex: &str) -> Vec<String> {
  let cleaned = regex
    .to_owned()
    .replace(r"\/", r"\\/")
    .replace(r"|)", r")?");

  let split = cleaned
    .split('|')
    .map(str::to_string)
    .collect::<Vec<String>>();

  println!("{:#?}", split);
  [0..split.len()].iter().map(|_| {
    let full_split = split.to_owned();
  });

  vec![cleaned]
}

impl UserAgentParser for Parser {
  type Item = Client;

  fn parse(&self, agent: impl std::string::ToString) -> Self::Item {
    unimplemented!()
  }
}

impl Parser {
  pub fn from_yaml(path: &str) -> Parser {
    let mut file = std::fs::File::open(path).expect("File not found!");
    Parser::from_file(file)
  }

  pub fn from_file(file: std::fs::File) -> Parser {
    let regex_file: RegexFile = serde_yaml::from_reader(file).expect("Serde Error");
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
