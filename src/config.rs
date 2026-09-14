use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_HISTORY_FILE: &str = "~/.zsh_history";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default)]
pub struct Config {
    pub matching: MatchingMode,
    pub max_suggestions: usize,
    pub case_sensitive: bool,
    pub history_file: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            matching: MatchingMode::Prefix,
            max_suggestions: 8,
            case_sensitive: false,
            history_file: DEFAULT_HISTORY_FILE.to_owned(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let mut config = Self::default();
        let Some(home) = home_dir() else {
            return config;
        };
        let path = home.join(".config/rrsreadline/config.toml");
        let Ok(contents) = fs::read_to_string(path) else {
            return config;
        };
        if let Ok(parsed) = toml::from_str::<Self>(&contents) {
            config = parsed;
        }
        config
    }

    pub fn history_path(&self) -> PathBuf {
        expand_tilde(&self.history_file)
    }
}

fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME").map(PathBuf::from)
}

fn expand_tilde(value: &str) -> PathBuf {
    if let Some(home) = home_dir() {
        if value == "~" {
            return home;
        }
        if let Some(rest) = value.strip_prefix("~/") {
            return home.join(rest);
        }
    }
    Path::new(value).to_path_buf()
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MatchingMode {
    #[default]
    Prefix,
    Contains,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_sensible() {
        let config = Config::default();
        assert_eq!(config.matching, MatchingMode::Prefix);
        assert_eq!(config.max_suggestions, 8);
        assert!(!config.case_sensitive);
        assert_eq!(config.history_file, "~/.zsh_history");
    }
}
