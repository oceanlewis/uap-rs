use super::*;

pub struct Matcher {
  regex: regex::Regex,
  family_replacement: Option<String>,
  device_replacement: Option<String>,
}

impl From<UserAgentParserEntry> for Matcher {
  fn from(entry: UserAgentParserEntry) -> Matcher {
    let regex_builder = regex::RegexBuilder::new(&entry.regex);
    let regex = regex_builder.build();

    if regex.is_err() {
      println!("{:#?}", entry.regex);
    }

    Matcher {
      regex: regex.expect("Regex failed to build"),
      family_replacement: entry.family_replacement,
      device_replacement: entry.device_replacement,
    }
  }
}
