use rand::random;
use serde::{Deserialize, Serialize};
use sled::Db;
use std::{cmp::Ordering, error::Error, fmt, time::SystemTime};

#[derive(Debug)]
pub enum NFAError {
    DatabaseError(sled::Error),
    SerializationError(bincode::Error),
    NoteNotFound,
}

impl Error for NFAError {}

impl fmt::Display for NFAError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NFAError::DatabaseError(e) => write!(f, "Database error: {}", e),
            NFAError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            NFAError::NoteNotFound => write!(f, "Note not found"),
        }
    }
}

impl From<sled::Error> for NFAError {
    fn from(err: sled::Error) -> NFAError {
        NFAError::DatabaseError(err)
    }
}

impl From<bincode::Error> for NFAError {
    fn from(err: bincode::Error) -> NFAError {
        NFAError::SerializationError(err)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

impl Eq for Note {}

impl Ord for Note {
    fn cmp(&self, other: &Self) -> Ordering {
        self.updated_at.cmp(&other.updated_at)
    }
}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct NoteManager {
    db: Db,
}

impl Note {
    pub fn new(title: String, content: String) -> Self {
        let now = SystemTime::now();

        Self {
            id: format!("{:x}", random::<u64>()),
            title,
            content,
            created_at: now,
            updated_at: now,
        }
    }
}

impl NoteManager {
    pub fn new(path: &str) -> Result<Self, NFAError> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    pub fn create_note(&self, title: String, content: String) -> Result<Note, NFAError> {
        let note = Note::new(title, content);
        let serialized = bincode::serialize(&note)?;
        self.db.insert(note.id.as_bytes(), serialized)?;
        Ok(note)
    }

    pub fn get_note(&self, id: &str) -> Result<Note, NFAError> {
        let data = self.db.get(id.as_bytes())?.ok_or(NFAError::NoteNotFound)?;

        let note: Note = bincode::deserialize(&data)?;
        Ok(note)
    }

    pub fn update_note(
        &self,
        id: &str,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<Note, NFAError> {
        let mut note = self.get_note(id)?;

        if let Some(title) = title {
            note.title = title;
        }

        if let Some(content) = content {
            note.content = content;
        }

        note.updated_at = SystemTime::now();
        let serialized = bincode::serialize(&note)?;
        self.db.insert(note.id.as_bytes(), serialized)?;

        Ok(note)
    }

    pub fn delete_note(&self, id: &str) -> Result<(), NFAError> {
        self.db.remove(id.as_bytes())?;
        Ok(())
    }

    pub fn list_notes(&self) -> Result<Vec<Note>, NFAError> {
        let mut notes = Vec::new();

        for item in self.db.iter() {
            let (_, value) = item?;
            let note: Note = bincode::deserialize(&value)?;
            notes.push(note);
        }

        // Sort notes by updated_at timestamp
        notes.sort_by(|a, b| a.updated_at.cmp(&b.updated_at));

        Ok(notes)
    }
}
