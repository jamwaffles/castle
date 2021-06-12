mod events;

use chrono::{DateTime, Utc};
use event_sauce::{Create, Entity, Event, Persistable, Storage};
pub use events::*;
use uuid::Uuid;

use crate::storage::FileStorage;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "note_type", content = "body")]
pub enum NoteBody {
    Text(String),
    // TODO: List, etc
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub session_id: Uuid,
    pub title: String,
    #[serde(flatten)]
    pub body: NoteBody,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Entity for Note {
    const ENTITY_TYPE: &'static str = "Note";

    fn entity_id(&self) -> Uuid {
        self.id
    }
}

impl Create<TextNoteCreated> for Note {
    fn create_from(event: &Event<TextNoteCreated>) -> Self {
        Self {
            id: event.entity_id,
            session_id: event.session_id,
            title: event.data.title.clone(),
            body: NoteBody::Text(event.data.body.clone()),
            created_at: event.created_at,
            updated_at: event.created_at,
        }
    }
}

impl Persistable<FileStorage> for Note {
    fn persist(self, storage: &mut FileStorage) -> Result<Self, <FileStorage as Storage>::Error> {
        storage.persist_note(&self);

        Ok(self)
    }
}
