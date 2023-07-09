use super::*;

#[derive(Debug, Display, From)]
pub enum Error {
    Regex(regex::Error),
}

#[derive(Debug)]
pub struct Matcher {
    regex: regex::bytes::Regex,
    family_replacement_has_group: bool,
    family_replacement: Option<String>,
    v1_replacement: Option<String>,
    v2_replacement: Option<String>,
    v3_replacement: Option<String>,
}

impl<'a> SubParser<'a> for Matcher {
    type Item = UserAgent<'a>;

    fn try_parse(&self, text: &'a str) -> Option<Self::Item> {
        if let Some(captures) = self.regex.captures(text.as_bytes()) {
            let family: Cow<'a, str> =
                if let Some(family_replacement) = &self.family_replacement {
                    replace_cow(
                        family_replacement,
                        self.family_replacement_has_group,
                        &captures,
                    )
                } else {
                    captures
                        .get(1)
                        .and_then(match_to_str)
                        .and_then(none_if_empty)
                        .map(Cow::Borrowed)?
                };

            let major: Option<Cow<'a, str>> = self
                .v1_replacement
                .as_ref()
                .map(|x| Cow::Owned(x.clone()))
                .or_else(|| {
                    captures
                        .get(2)
                        .and_then(match_to_str)
                        .and_then(none_if_empty)
                        .map(Cow::Borrowed)
                });

            let minor: Option<Cow<'a, str>> = self
                .v2_replacement
                .as_ref()
                .map(|x| Cow::Owned(x.clone()))
                .or_else(|| {
                    captures
                        .get(3)
                        .and_then(match_to_str)
                        .and_then(none_if_empty)
                        .map(Cow::Borrowed)
                });

            let patch: Option<Cow<'a, str>> = self
                .v3_replacement
                .as_ref()
                .map(|x| Cow::Owned(x.clone()))
                .or_else(|| {
                    captures
                        .get(4)
                        .and_then(match_to_str)
                        .and_then(none_if_empty)
                        .map(Cow::Borrowed)
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
    pub fn try_from(
        entry: UserAgentParserEntry,
        unicode: bool,
    ) -> Result<Matcher, Error> {
        let regex = regex::bytes::RegexBuilder::new(&clean_escapes(&entry.regex))
            .unicode(unicode)
            .size_limit(20 * (1 << 20))
            .build();

        Ok(Matcher {
            regex: regex?,
            family_replacement_has_group: entry
                .family_replacement
                .as_ref()
                .map_or(false, |x| has_group(x.as_str())),
            family_replacement: entry.family_replacement,
            v1_replacement: entry.v1_replacement,
            v2_replacement: entry.v2_replacement,
            v3_replacement: entry.v3_replacement,
        })
    }
}
