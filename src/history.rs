//! Shell-independent history representation.

use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct History {
    entries: Vec<String>,
}

impl History {
    pub fn new(entries: impl IntoIterator<Item = String>) -> Self {
        Self {
            entries: entries.into_iter().collect(),
        }
    }

    pub fn entries(&self) -> &[String] {
        &self.entries
    }

    pub fn remove_all(&mut self, target: &str) {
        self.entries.retain(|entry| entry != target);
    }
}

/// Loads the common line-oriented representation emitted by Zsh history.
///
/// With `EXTENDED_HISTORY`, Zsh prefixes entries with `: timestamp:duration;`.
/// Plain history lines are accepted as well so this remains useful for other
/// shells that store one command per line.
pub fn load_file(path: &Path) -> io::Result<History> {
    let contents = fs::read_to_string(path)?;
    let entries = contents
        .lines()
        .filter_map(parse_line)
        .map(str::to_owned)
        .collect::<Vec<_>>();
    Ok(History::new(entries))
}

fn parse_line(line: &str) -> Option<&str> {
    let command = if line.starts_with(": ") {
        line.split_once(';')?.1
    } else {
        line
    };
    (!command.is_empty()).then_some(command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_plain_and_extended_history() {
        assert_eq!(parse_line("git status"), Some("git status"));
        assert_eq!(parse_line(": 1710000000:0;cargo test"), Some("cargo test"));
        assert_eq!(parse_line(": 1710000000:0;"), None);
    }
}
