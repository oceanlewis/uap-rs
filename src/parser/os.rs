use super::*;

#[derive(Debug, Display, From)]
pub enum Error {
    Onig(onig::Error),
}

#[derive(Debug)]
pub struct Matcher {
    regex: onig::Regex,
    os_replacement: Option<String>,
    os_v1_replacement: Option<String>,
    os_v2_replacement: Option<String>,
    os_v3_replacement: Option<String>,
}

impl SubParser for Matcher {
    type Item = OS;

    fn try_parse(&self, text: &str) -> Option<Self::Item> {
        if let Some(captures) = self.regex.captures(text) {
            let family: String = if let Some(os_replacement) = &self.os_replacement {
                replace(&os_replacement, &captures)
            } else {
                captures.at(1).map(str::to_string)?
            };

            let major: Option<String> =
                if let Some(os_v1_replacement) = &self.os_v1_replacement {
                    let replaced = replace(&os_v1_replacement, &captures);
                    empty_string_is_none(&replaced)
                } else {
                    captures.at(2).map(str::to_string)
                };

            let minor: Option<String> =
                if let Some(os_v2_replacement) = &self.os_v2_replacement {
                    let replaced = replace(&os_v2_replacement, &captures);
                    empty_string_is_none(&replaced)
                } else {
                    captures.at(3).map(str::to_string)
                };

            let patch: Option<String> =
                if let Some(os_v3_replacement) = &self.os_v3_replacement {
                    let replaced = replace(&os_v3_replacement, &captures);
                    empty_string_is_none(&replaced)
                } else {
                    captures.at(4).map(str::to_string)
                };

            let patch_minor: Option<String> = captures.at(5).map(str::to_string);

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
        let regex = onig::Regex::new(&entry.regex);

        Ok(Matcher {
            regex: regex?,
            os_replacement: entry.os_replacement,
            os_v1_replacement: entry.os_v1_replacement,
            os_v2_replacement: entry.os_v2_replacement,
            os_v3_replacement: entry.os_v3_replacement,
        })
    }
}
