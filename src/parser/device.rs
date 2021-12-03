use super::*;

#[derive(Debug, Display, From)]
pub enum Error {
    Regex(regex::Error),
}

#[derive(Debug)]
pub struct Matcher {
    regex: regex::Regex,
    device_replacement: Option<String>,
    brand_replacement: Option<String>,
    model_replacement: Option<String>,
}

impl SubParser for Matcher {
    type Item = Device;

    fn try_parse(&self, text: &str) -> Option<Self::Item> {
        if !self.regex.is_match(text) {
            return None;
        }

        if let Some(captures) = self.regex.captures(text) {
            let family: String =
                if let Some(device_replacement) = &self.device_replacement {
                    replace(device_replacement, &captures)
                } else {
                    captures
                        .get(1)
                        .map(|x| x.as_str())
                        .and_then(none_if_empty)
                        .map(ToString::to_string)?
                };

            let brand: Option<String> =
                if let Some(brand_replacement) = &self.brand_replacement {
                    none_if_empty(replace(brand_replacement, &captures))
                } else {
                    None
                };

            let model: Option<String> =
                if let Some(model_replacement) = &self.model_replacement {
                    none_if_empty(replace(model_replacement, &captures))
                } else {
                    captures
                        .get(1)
                        .map(|x| x.as_str())
                        .and_then(none_if_empty)
                        .map(ToString::to_string)
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
        let regex_with_flags =
            if !entry.regex_flag.as_ref().map_or(true, String::is_empty) {
                format!("(?{}){}", entry.regex_flag.unwrap_or_default(), entry.regex)
            } else {
                entry.regex
            };
        let regex = regex::RegexBuilder::new(&clean_escapes(&regex_with_flags))
            .size_limit(20 * (1 << 20))
            .build();

        Ok(Matcher {
            regex: regex?,
            device_replacement: entry.device_replacement,
            brand_replacement: entry.brand_replacement,
            model_replacement: entry.model_replacement,
        })
    }
}
