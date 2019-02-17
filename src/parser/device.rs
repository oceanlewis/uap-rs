use super::*;

#[derive(Debug)]
pub struct Matcher {
  regex: onig::Regex,
  device_replacement: Option<String>,
  brand_replacement: Option<String>,
  model_replacement: Option<String>,
}

// fn replace(replacement: &String, match: String) -> String
// {
// def replace(replacement: String, matcher: Matcher): String = {
//    (if (replacement.contains("$") && matcher.groupCount() >= 1)  {
//      (1 to matcher.groupCount()).foldLeft(replacement)((rep, i) => {
//        val toInsert = if (matcher.group(i) ne null) matcher.group(i) else ""
//        rep.replaceFirst("\\$" + i, Matcher.quoteReplacement(toInsert))
//      })
//    } else replacement).trim
//  }
// }

fn replace(replacement: &str, captures: &onig::Captures) -> String {
  let dollar_signs =
    replacement.chars().fold(
      0,
      |instances: usize, c| {
        if c == '$' {
          instances + 1
        } else {
          instances
        }
      },
    );

  if replacement.contains('$') && !captures.is_empty() {
    (1..=captures.len())
      .fold(replacement.to_owned(), |mut state: String, i: usize| {
        let group = captures.at(i).unwrap_or_default();
        state.replace(&format!("${}", i), &group)
      })
      .trim()
      .to_owned()
  } else {
    replacement.to_owned()
  }
}

impl SubParser for Matcher {
  type Item = Device;

  fn try_parse(&self, text: &str) -> Option<Self::Item> {
    if let Some(captures) = self.regex.captures(text) {
      let device_family: String =
        if let Some(device_replacement) = &self.device_replacement {
          replace(&device_replacement, &captures)
        } else {
          captures.at(1).map(str::to_string)?
        };

      let brand: Option<String> = if let Some(brand_replacement) = &self.brand_replacement
      {
        let replaced = replace(&brand_replacement, &captures);
        empty_string_is_none(&replaced)
      } else {
        captures.at(2).map(str::to_string)
      };

      let model: Option<String> = if let Some(model_replacement) = &self.model_replacement
      {
        let replaced = replace(&model_replacement, &captures);
        empty_string_is_none(&replaced)
      } else {
        captures.at(3).map(str::to_string)
      };

      let device = Device {
        family: device_family,
        brand: brand,
        model: model,
      };

      Some(device)
    } else {
      None
    }
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

    Matcher {
      regex: regex
        .unwrap_or_else(|_| panic!("Regex:\n{:#?}\nfailed to build", entry.regex)),
      device_replacement: entry.device_replacement,
      brand_replacement: entry.brand_replacement,
      model_replacement: entry.model_replacement,
    }
  }
}
