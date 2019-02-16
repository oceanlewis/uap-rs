use super::*;

#[derive(Debug)]
pub struct Matcher {
  regex: onig::Regex,
  device_replacement: Option<String>,
  brand_replacement: Option<String>,
  model_replacement: Option<String>,
}

impl SubParser for Matcher {
  type Item = Device;

  fn try_parse(&self, text: &str) -> Option<Self::Item> {
    if let Some(captures) = self.regex.captures(text) {
      //pub struct Device {
      //  family: DeviceFamily,
      //  brand: Option<DeviceBrand>,
      //  model: Option<DeviceModel>,
      //}
      // pub struct DeviceParserEntry {
      //   pub regex_flag: Option<String>,
      //   pub regex: String,
      //   pub device_replacement: Option<String>,
      //   pub brand_replacement: Option<String>,
      //   pub model_replacement: Option<String>,
      // }

      // if let Some(family) = self
      //   .family_replacement
      //   .to_owned()
      //   .or_else(|| captures.at(0).map(String::from))
      // {
      //   return Some(Device {
      //     family: family,
      //     major: captures.at(2).map(str::to_string),
      //     minor: captures.at(3).map(str::to_string),
      //     patch: captures.at(4).map(str::to_string),
      //   });
      // }
    }

    None
  }
}

impl From<DeviceParserEntry> for Matcher {
  fn from(entry: DeviceParserEntry) -> Matcher {
    let options = if (Some("i") == entry.regex_flag.as_ref().map(String::as_str)) {
      onig::RegexOptions::REGEX_OPTION_IGNORECASE
    } else {
      onig::RegexOptions::REGEX_OPTION_NONE
    };
    let regex = onig::Regex::with_options(&entry.regex, options, onig::Syntax::default());

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
