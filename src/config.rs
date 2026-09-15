use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_ZSH_HISTORY_FILE: &str = "~/.zsh_history";
const DEFAULT_BASH_HISTORY_FILE: &str = "~/.bash_history";

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
            max_suggestions: 10,
            case_sensitive: false,
            history_file: DEFAULT_ZSH_HISTORY_FILE.to_owned(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        Self::load_for_shell("zsh")
    }

    pub fn load_for_shell(shell: &str) -> Self {
        let mut config = Self::for_shell(shell);
        let Some(home) = home_dir() else {
            return config;
        };
        let path = home.join(".config/rrsreadline/config.toml");
        let Ok(contents) = fs::read_to_string(path) else {
            return config;
        };
        let Ok(table) = toml::from_str::<toml::Value>(&contents) else {
            return config;
        };
        if let Some(value) = table.get("matching")
            && let Ok(parsed) = value.clone().try_into()
        {
            config.matching = parsed;
        }
        if let Some(value) = table.get("max_suggestions")
            && let Some(parsed) = value.as_integer().and_then(|n| usize::try_from(n).ok())
        {
            config.max_suggestions = parsed;
        }
        if let Some(value) = table.get("case_sensitive")
            && let Some(parsed) = value.as_bool()
        {
            config.case_sensitive = parsed;
        }
        if let Some(value) = table.get("history_file")
            && let Some(parsed) = value.as_str()
        {
            config.history_file = parsed.to_owned();
        }
        config
    }

    fn for_shell(shell: &str) -> Self {
        let mut config = Self::default();
        if shell == "bash" {
            config.history_file = DEFAULT_BASH_HISTORY_FILE.to_owned();
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
        assert_eq!(config.max_suggestions, 10);
        assert!(!config.case_sensitive);
        assert_eq!(config.history_file, DEFAULT_ZSH_HISTORY_FILE);
    }

    #[test]
    fn bash_uses_bash_history_by_default() {
        assert_eq!(
            Config::for_shell("bash").history_file,
            DEFAULT_BASH_HISTORY_FILE
        );
    }
}
