use super::*;

pub struct Matcher {
  regex: Vec<onig::Regex>,
  os_replacement: Option<String>,
  os_v1_replacement: Option<String>,
  os_v2_replacement: Option<String>,
  os_v3_replacement: Option<String>,
}

impl From<OSParserEntry> for Matcher {
  fn from(entry: OSParserEntry) -> Matcher {
    let fixed_regexes = split_regexes(&entry.regex);

    let regexes = fixed_regexes
      .into_iter()
      .map(|regex| {
        let regex_builder = onig::RegexBuilder::new(&regex);

        let regex = regex_builder.build();
        if regex.is_err() {
          println!("{:#?}", entry.regex);
        }

        regex.expect("Regex failed to build")
      })
      .collect();

    Matcher {
      regex: regexes,
      os_replacement: entry.os_replacement,
      os_v1_replacement: entry.os_v1_replacement,
      os_v2_replacement: entry.os_v2_replacement,
      os_v3_replacement: entry.os_v3_replacement,
    }
  }
}
