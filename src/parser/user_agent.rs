use super::*;

#[derive(Debug)]
pub struct Matcher {
    regex: onig::Regex,
    family_replacement: Option<String>,
    device_replacement: Option<String>,
}

impl SubParser for Matcher {
    type Item = UserAgent;

    fn try_parse(&self, text: &str) -> Option<Self::Item> {
        if let Some(captures) = self.regex.captures(text) {
            if captures.at(0).is_none() {
                return None;
            }

            let family = self
                .family_replacement
                .to_owned()
                .or_else(|| captures.at(1).map(String::from))
                .expect("should have matched");

            if self.family_replacement.is_some() {
                println!("replacement: {:#?}", self.family_replacement);
            }

            Some(UserAgent {
                family: family,
                major: captures.at(2).map(str::to_string),
                minor: captures.at(3).map(str::to_string),
                patch: captures.at(4).map(str::to_string),
            })
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
            device_replacement: entry.device_replacement,
        }
    }
}
