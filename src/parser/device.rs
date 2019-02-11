use super::*;

#[derive(Debug)]
pub struct Matcher {
  regex: onig::Regex,
  case_insensitive: bool,
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
      println!("{:#?}", captures.at(0));
      println!("{:#?}", captures.at(1));
      println!("{:#?}", captures.at(2));
      println!("{:#?}", captures.at(3));

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
    let regex = onig::Regex::new(&entry.regex);

    if regex.is_err() {
      println!("{:#?}", entry.regex);
    }

    Matcher {
      regex: regex.expect("Regex failed to build"),
      case_insensitive: (Some("i") == entry.regex_flag.as_ref().map(String::as_str)),
      device_replacement: entry.device_replacement,
      brand_replacement: entry.brand_replacement,
      model_replacement: entry.model_replacement,
    }
  }
}
