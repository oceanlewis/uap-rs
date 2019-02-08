use super::*;

#[derive(Debug)]
pub struct Matcher {
  regex: onig::Regex,
  family_replacement: Option<String>,
  device_replacement: Option<String>,
}

impl From<UserAgentParserEntry> for Matcher {
  fn from(entry: UserAgentParserEntry) -> Matcher {
    let regex = onig::Regex::new(&entry.regex);

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
