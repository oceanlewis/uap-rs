use super::*;

pub struct Matcher {
  regex: Vec<onig::Regex>,
  device_replacement: Option<String>,
  brand_replacement: Option<String>,
  model_replacement: Option<String>,
}

impl From<DeviceParserEntry> for Matcher {
  fn from(entry: DeviceParserEntry) -> Matcher {
    let case_insensitive = (entry.regex_flag == Some("i".to_string()));
    let fixed_regexes = split_regexes(&entry.regex);

    let regexes = fixed_regexes
      .into_iter()
      .map(|regex| {
        let mut regex_builder = onig::RegexBuilder::new(&regex);
        regex_builder.case_insensitive(case_insensitive);

        let regex = regex_builder.build();
        if regex.is_err() {
          println!("{:#?}", entry.regex);
        }

        regex.expect("Regex failed to build")
      })
      .collect();

    Matcher {
      regex: regexes,
      device_replacement: entry.device_replacement,
      brand_replacement: entry.brand_replacement,
      model_replacement: entry.model_replacement,
    }
  }
}
