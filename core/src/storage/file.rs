//! Very simple, file-backed storage.

use crate::dao::note::Note;
use event_sauce::{DBEvent, Entity, EventData, Persistable, Persister, Storage};
use std::{
    convert::TryInto,
    fs::File,
    io::{self, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, thiserror::Error)]
pub enum FileStorageError {
    #[error("could not open file for reading/writing: {0}")]
    FileOpen(#[from] io::Error),

    #[error("file contains invalid data: {0}")]
    DecodeError(#[from] serde_json::Error),
}

#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct Items {
    #[serde(default)]
    events: Vec<DBEvent>,

    #[serde(default)]
    pub notes: Vec<Note>,
}

#[derive(Debug)]
pub struct FileStorage {
    path: PathBuf,

    pub items: Items,
}

impl FileStorage {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, FileStorageError> {
        let path = path.as_ref();

        // TODO: Use try_exists() as well when it's stabilised
        let items = if path.is_file() {
            let mut f = File::open(path)?;

            let items: Items = serde_json::from_reader(&mut f)?;

            items
        } else {
            Items::default()
        };

        Ok(Self {
            path: path.into(),
            items,
        })
    }

    /// Create or update a note in memory.
    pub fn persist_note(&mut self, note: &Note) {
        self.items.notes.push(note.clone());
    }

    fn write_to_disk(&self) -> Result<(), FileStorageError> {
        let f = File::create(&self.path)?;

        serde_json::to_writer_pretty(f, &self.items)?;

        Ok(())
    }
}

impl Storage for FileStorage {
    type Error = FileStorageError;
}

impl<D, E> Persistable<FileStorage, E> for Persister<D, E>
where
    D: EventData,
    E: Entity + Persistable<FileStorage>,
{
    fn persist(self, storage: &mut FileStorage) -> Result<E, <FileStorage as Storage>::Error> {
        let event: DBEvent = self.event.try_into().expect("Failed to convert event");

        storage.items.events.push(event);

        let entity = self.entity.persist(storage)?;

        storage.write_to_disk()?;

        Ok(entity)
    }
}
