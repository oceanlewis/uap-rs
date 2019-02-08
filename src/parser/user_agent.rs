use super::*;

pub struct Matcher {
  regex: Vec<onig::Regex>,
  family_replacement: Option<String>,
  device_replacement: Option<String>,
}

impl From<UserAgentParserEntry> for Matcher {
  fn from(entry: UserAgentParserEntry) -> Matcher {
    let fixed_regexes = split_regexes(&entry.regex);

    let regexes = fixed_regexes
      .into_iter()
      .map(|regex| {
        let regex_builder = onig::Regex::new(&regex);

        let regex = regex_builder.build();
        if regex.is_err() {
          println!("{:#?}", entry.regex);
        }

        regex.expect("Regex failed to build")
      })
      .collect();

    Matcher {
      regex: regexes,
      family_replacement: entry.family_replacement,
      device_replacement: entry.device_replacement,
    }
  }
}
