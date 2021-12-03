use super::*;

#[derive(Debug, Display, From)]
pub enum Error {
    Regex(regex::Error),
}

#[derive(Debug)]
pub struct Matcher {
    regex: regex::Regex,
    os_replacement: Option<String>,
    os_v1_replacement: Option<String>,
    os_v2_replacement: Option<String>,
    os_v3_replacement: Option<String>,
}

impl SubParser for Matcher {
    type Item = OS;

    fn try_parse(&self, text: &str) -> Option<Self::Item> {
        if !self.regex.is_match(text) {
            return None;
        }

        if let Some(captures) = self.regex.captures(text) {
            let family: String = if let Some(os_replacement) = &self.os_replacement {
                replace(os_replacement, &captures)
            } else {
                captures
                    .get(1)
                    .map(|x| x.as_str())
                    .and_then(none_if_empty)
                    .map(ToString::to_string)?
            };

            let major: Option<String> =
                if let Some(os_v1_replacement) = &self.os_v1_replacement {
                    none_if_empty(replace(os_v1_replacement, &captures))
                } else {
                    captures
                        .get(2)
                        .map(|x| x.as_str())
                        .and_then(none_if_empty)
                        .map(ToString::to_string)
                };

            let minor: Option<String> =
                if let Some(os_v2_replacement) = &self.os_v2_replacement {
                    none_if_empty(replace(os_v2_replacement, &captures))
                } else {
                    captures
                        .get(3)
                        .map(|x| x.as_str())
                        .and_then(none_if_empty)
                        .map(ToString::to_string)
                };

            let patch: Option<String> =
                if let Some(os_v3_replacement) = &self.os_v3_replacement {
                    none_if_empty(replace(os_v3_replacement, &captures))
                } else {
                    captures
                        .get(4)
                        .map(|x| x.as_str())
                        .and_then(none_if_empty)
                        .map(ToString::to_string)
                };

            let patch_minor: Option<String> = captures
                .get(5)
                .map(|x| x.as_str())
                .and_then(none_if_empty)
                .map(ToString::to_string);

            Some(OS {
                family,
                major,
                minor,
                patch,
                patch_minor,
            })
        } else {
            None
        }
    }
}

impl Matcher {
    pub fn try_from(entry: OSParserEntry) -> Result<Matcher, Error> {
        let regex = regex::Regex::new(&clean_escapes(&entry.regex));

        Ok(Matcher {
            regex: regex?,
            os_replacement: entry.os_replacement,
            os_v1_replacement: entry.os_v1_replacement,
            os_v2_replacement: entry.os_v2_replacement,
            os_v3_replacement: entry.os_v3_replacement,
        })
    }
}
