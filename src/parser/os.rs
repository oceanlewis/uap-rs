use super::*;

#[derive(Debug)]
pub struct Matcher {
  regex: onig::Regex,
  os_replacement: Option<String>,
  v1_replacement: Option<String>,
  v2_replacement: Option<String>,
  v3_replacement: Option<String>,
}

impl From<OSParserEntry> for Matcher {
  fn from(entry: OSParserEntry) -> Matcher {
    let regex = onig::Regex::new(&entry.regex);

    if regex.is_err() {
      println!("{:#?}", entry.regex);
    }

    Matcher {
      regex: regex.expect("Regex failed to build"),
      os_replacement: entry.os_replacement,
      v1_replacement: entry.v1_replacement,
      v2_replacement: entry.v2_replacement,
      v3_replacement: entry.v3_replacement,
    }
  }
}
