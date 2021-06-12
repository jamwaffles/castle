use super::Note;
use event_sauce::EventData;
use serde::{Deserialize, Serialize};

/// Create a note with a text-only body.
#[derive(Debug, Serialize, Deserialize)]
pub struct TextNoteCreated {
    pub title: String,
    pub body: String,
}

impl EventData for TextNoteCreated {
    type Entity = Note;

    fn event_type() -> &'static str {
        "TextNoteCreated"
    }
}
