use super::*;

pub struct Matcher {
  regex: onig::Regex,
  device_replacement: Option<String>,
  brand_replacement: Option<String>,
  model_replacement: Option<String>,
}

impl From<DeviceParserEntry> for Matcher {
  fn from(entry: DeviceParserEntry) -> Matcher {
    let regex = onig::Regex::new(&entry.regex);

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
