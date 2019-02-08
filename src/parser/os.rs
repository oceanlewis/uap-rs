use super::*;

pub struct Matcher {
  regex: regex::Regex,
  os_replacement: Option<String>,
  os_v1_replacement: Option<String>,
  os_v2_replacement: Option<String>,
  os_v3_replacement: Option<String>,
}

impl From<OSParserEntry> for Matcher {
  fn from(entry: OSParserEntry) -> Matcher {
    let regex_builder = regex::RegexBuilder::new(&entry.regex);
    let regex = regex_builder.build();

    if regex.is_err() {
      println!("{:#?}", entry.regex);
    }

    Matcher {
      regex: regex.expect("Regex failed to build"),
      os_replacement: entry.os_replacement,
      os_v1_replacement: entry.os_v1_replacement,
      os_v2_replacement: entry.os_v2_replacement,
      os_v3_replacement: entry.os_v3_replacement,
    }
  }
}
