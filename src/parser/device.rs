use super::*;

pub struct Matcher {
  regex: regex::Regex,
  device_replacement: Option<String>,
  brand_replacement: Option<String>,
  model_replacement: Option<String>,
}

impl From<DeviceParserEntry> for Matcher {
  fn from(entry: DeviceParserEntry) -> Matcher {
    let case_insensitive = (entry.regex_flag == Some("i".to_string()));
    let mut regex_builder = regex::RegexBuilder::new(&entry.regex);
    regex_builder.case_insensitive(case_insensitive);

    let regex = regex_builder.build();

    if regex.is_err() {
      println!("{:#?}", entry.regex);
    }

    Matcher {
      regex: regex.expect("Regex failed to build"),
      device_replacement: entry.device_replacement,
      brand_replacement: entry.brand_replacement,
      model_replacement: entry.model_replacement,
    }
  }
}
