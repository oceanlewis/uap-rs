use super::*;

#[derive(Debug, Display, From)]
pub enum Error {
    Onig(onig::Error),
}

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
            let family: String =
                if let Some(device_replacement) = &self.device_replacement {
                    replace(&device_replacement, &captures)
                } else {
                    captures.at(1).map(str::to_string)?
                };

            let brand: Option<String> =
                if let Some(brand_replacement) = &self.brand_replacement {
                    let replaced = replace(&brand_replacement, &captures);
                    empty_string_is_none(&replaced)
                } else {
                    captures.at(2).map(str::to_string)
                };

            let model: Option<String> =
                if let Some(model_replacement) = &self.model_replacement {
                    let replaced = replace(&model_replacement, &captures);
                    empty_string_is_none(&replaced)
                } else {
                    captures.at(3).map(str::to_string)
                };

            Some(Device {
                family,
                brand,
                model,
            })
        } else {
            None
        }
    }
}

impl Matcher {
    pub fn try_from(entry: DeviceParserEntry) -> Result<Matcher, Error> {
        let options = if Some("i") == entry.regex_flag.as_ref().map(String::as_str) {
            onig::RegexOptions::REGEX_OPTION_IGNORECASE
        } else {
            onig::RegexOptions::REGEX_OPTION_NONE
        };

        let regex =
            onig::Regex::with_options(&entry.regex, options, onig::Syntax::default());

        Ok(Matcher {
            regex: regex?,
            device_replacement: entry.device_replacement,
            brand_replacement: entry.brand_replacement,
            model_replacement: entry.model_replacement,
        })
    }
}
