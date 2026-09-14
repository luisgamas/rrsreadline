use crate::matching::Suggestion;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorEvent {
    TextChanged,
    MoveUp,
    MoveDown,
    Accept,
    Cancel,
    DeleteSelected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineOutput {
    Suggestions(Vec<Suggestion>),
    Accepted(String),
    Cancelled,
    Delete(String),
}

#[derive(Debug, Default)]
pub struct EditorState {
    selected: Option<usize>,
}

impl EditorState {
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn handle(
        &mut self,
        event: EditorEvent,
        suggestions: &[Suggestion],
    ) -> Option<EngineOutput> {
        match event {
            EditorEvent::TextChanged => {
                self.selected = None;
                None
            }
            EditorEvent::MoveUp => {
                self.selected = previous(self.selected, suggestions.len());
                None
            }
            EditorEvent::MoveDown => {
                self.selected = next(self.selected, suggestions.len());
                None
            }
            EditorEvent::Accept => self
                .selected
                .and_then(|index| suggestions.get(index))
                .map(|suggestion| EngineOutput::Accepted(suggestion.text.clone())),
            EditorEvent::Cancel => {
                self.selected = None;
                Some(EngineOutput::Cancelled)
            }
            EditorEvent::DeleteSelected => self
                .selected
                .and_then(|index| suggestions.get(index))
                .map(|suggestion| EngineOutput::Delete(suggestion.text.clone())),
        }
    }
}

fn previous(selected: Option<usize>, count: usize) -> Option<usize> {
    if count == 0 {
        None
    } else {
        Some(selected.unwrap_or(0).checked_sub(1).unwrap_or(count - 1))
    }
}

fn next(selected: Option<usize>, count: usize) -> Option<usize> {
    if count == 0 {
        None
    } else {
        Some(selected.map_or(0, |index| (index + 1) % count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn suggestions() -> Vec<Suggestion> {
        ["git status", "git diff"]
            .into_iter()
            .map(|text| Suggestion { text: text.into() })
            .collect()
    }

    #[test]
    fn navigation_wraps() {
        let mut state = EditorState::default();
        let items = suggestions();
        state.handle(EditorEvent::MoveDown, &items);
        assert_eq!(state.selected(), Some(0));
        state.handle(EditorEvent::MoveUp, &items);
        assert_eq!(state.selected(), Some(1));
    }
}
