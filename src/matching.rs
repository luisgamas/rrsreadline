use crate::config::MatchingMode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suggestion {
    pub text: String,
}

#[derive(Debug, Clone, Copy)]
pub struct Matcher {
    mode: MatchingMode,
    case_sensitive: bool,
    max_suggestions: usize,
}

impl Matcher {
    pub fn new(mode: MatchingMode, case_sensitive: bool, max_suggestions: usize) -> Self {
        Self {
            mode,
            case_sensitive,
            max_suggestions,
        }
    }

    pub fn suggest(&self, entries: &[String], query: &str) -> Vec<Suggestion> {
        if query.is_empty() || self.max_suggestions == 0 {
            return Vec::new();
        }

        let normalized_query = self.normalize(query);
        entries
            .iter()
            .rev()
            .filter(|entry| {
                let candidate = self.normalize(entry);
                match self.mode {
                    MatchingMode::Prefix => candidate.starts_with(normalized_query.as_ref()),
                    MatchingMode::Contains => candidate.contains(normalized_query.as_ref()),
                }
            })
            .take(self.max_suggestions)
            .map(|text| Suggestion { text: text.clone() })
            .collect()
    }

    fn normalize<'a>(&self, value: &'a str) -> std::borrow::Cow<'a, str> {
        if self.case_sensitive {
            std::borrow::Cow::Borrowed(value)
        } else {
            std::borrow::Cow::Owned(value.to_lowercase())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entries() -> Vec<String> {
        ["git status", "cargo test", "git diff", "git status"]
            .into_iter()
            .map(str::to_owned)
            .collect()
    }

    #[test]
    fn returns_newest_matching_entries_first() {
        let matcher = Matcher::new(MatchingMode::Prefix, false, 3);
        let result = matcher.suggest(&entries(), "git");
        assert_eq!(
            result.into_iter().map(|item| item.text).collect::<Vec<_>>(),
            vec!["git status", "git diff", "git status"]
        );
    }

    #[test]
    fn supports_contains_matching() {
        let matcher = Matcher::new(MatchingMode::Contains, false, 8);
        assert_eq!(matcher.suggest(&entries(), "TEST")[0].text, "cargo test");
    }
}
