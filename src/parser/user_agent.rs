use super::*;

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
            // if captures.at(0).is_none() {
            //     return None;
            // }

            // let fam_rep = captures.at(1).map(str::to_string);
            // let major_rep = captures.at(2).map(str::to_string);
            // let minor_rep = captures.at(3).map(str::to_string);
            // let patch_rep = captures.at(4).map(str::to_string);

            let family: String = if let Some(mut family_replacement) =
                self.family_replacement.to_owned()
            {
                if family_replacement.contains(r"$1") && captures.len() > 1 {
                    family_replacement =
                        family_replacement.replace("$1", captures.at(1).unwrap());
                }

                Some(family_replacement)
            } else {
                captures.at(1).map(String::from)
            }?;

            let major = self
                .v1_replacement
                .to_owned()
                .or_else(|| captures.at(2).map(str::to_string))
                .and_then(|s| str::parse::<usize>(&s).ok());

            let minor = self
                .v2_replacement
                .to_owned()
                .or_else(|| captures.at(3).map(str::to_string))
                .and_then(|s| str::parse::<usize>(&s).ok());

            let patch = self
                .v3_replacement
                .to_owned()
                .or_else(|| captures.at(4).map(str::to_string))
                .and_then(|s| str::parse::<usize>(&s).ok());

            let agent = UserAgent {
                family: family.to_owned(),
                major: major,
                minor: minor,
                patch: patch,
            };

            Some(agent)
        } else {
            None
        }
    }
}

impl From<UserAgentParserEntry> for Matcher {
    fn from(entry: UserAgentParserEntry) -> Matcher {
        let regex = onig::Regex::new(&entry.regex);

        if regex.is_err() {
            println!("{:#?}", entry.regex);
        }

        Matcher {
            regex: regex.expect("Regex failed to build"),
            family_replacement: entry.family_replacement,
            v1_replacement: entry.v1_replacement,
            v2_replacement: entry.v2_replacement,
            v3_replacement: entry.v3_replacement,
        }
    }
}
