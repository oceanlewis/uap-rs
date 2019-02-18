use super::*;

#[derive(Debug, Display, From)]
pub enum Error {
    Onig(onig::Error),
}

#[derive(Debug)]
pub struct Matcher {
    regex: onig::Regex,
    family_replacement: Option<String>,
    v1_replacement: Option<String>,
    v2_replacement: Option<String>,
    v3_replacement: Option<String>,
}

impl SubParser for Matcher {
    type Item = UserAgent;

    fn try_parse(&self, text: &str) -> Option<Self::Item> {
        if let Some(captures) = self.regex.captures(text) {
            let family: String =
                if let Some(family_replacement) = &self.family_replacement {
                    replace(&family_replacement, &captures)
                } else {
                    captures.at(1).map(str::to_string)?
                }
                .to_owned();

            let major = self.v1_replacement.to_owned().or_else(|| {
                captures
                    .at(2)
                    .and_then(none_if_str_is_empty)
                    .map(str::to_string)
            });

            let minor = self.v2_replacement.to_owned().or_else(|| {
                captures
                    .at(3)
                    .and_then(none_if_str_is_empty)
                    .map(str::to_string)
            });

            let patch = self.v3_replacement.to_owned().or_else(|| {
                captures
                    .at(4)
                    .and_then(none_if_str_is_empty)
                    .map(str::to_string)
            });

            Some(UserAgent {
                family,
                major,
                minor,
                patch,
            })
        } else {
            None
        }
    }
}

impl Matcher {
    pub fn try_from(entry: UserAgentParserEntry) -> Result<Matcher, Error> {
        let regex = onig::Regex::new(&entry.regex);

        Ok(Matcher {
            regex: regex?,
            family_replacement: entry.family_replacement,
            v1_replacement: entry.v1_replacement,
            v2_replacement: entry.v2_replacement,
            v3_replacement: entry.v3_replacement,
        })
    }
}
