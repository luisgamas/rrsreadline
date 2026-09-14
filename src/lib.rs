//! Portable suggestion engine for rRsReadLine.

pub mod config;
pub mod history;
pub mod matching;
pub mod shell;
pub mod state;

pub use config::MatchingMode;
pub use matching::{Matcher, Suggestion};
pub use state::{EditorEvent, EditorState, EngineOutput};
