#[cfg(test)]
mod tests {
    use nfa::{NFAError, NoteManager};
    use std::time::Duration;
    use tempfile::tempdir;

    fn setup() -> (NoteManager, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.db");
        let manager = NoteManager::new(path.to_str().unwrap()).unwrap();
        (manager, dir)
    }

    #[test]
    fn test_create_and_get_note() {
        let (manager, _dir) = setup();

        let title = "Test Note";
        let content = "Test Content";

        let created_note = manager
            .create_note(title.to_string(), content.to_string())
            .unwrap();
        let retrieved_note = manager.get_note(&created_note.id).unwrap();

        assert_eq!(created_note, retrieved_note);
        assert_eq!(retrieved_note.title, title);
        assert_eq!(retrieved_note.content, content);
    }

    #[test]
    fn test_update_note() {
        let (manager, _dir) = setup();

        let note = manager
            .create_note("Original".to_string(), "Content".to_string())
            .unwrap();

        // Small delay to ensure updated_at will be different
        std::thread::sleep(Duration::from_millis(10));

        let updated = manager
            .update_note(
                &note.id,
                Some("Updated Title".to_string()),
                Some("Updated Content".to_string()),
            )
            .unwrap();

        assert_eq!(updated.title, "Updated Title");
        assert_eq!(updated.content, "Updated Content");
        assert!(updated.updated_at > note.created_at);
    }

    #[test]
    fn test_delete_note() {
        let (manager, _dir) = setup();

        let note = manager
            .create_note("Title".to_string(), "Content".to_string())
            .unwrap();
        manager.delete_note(&note.id).unwrap();

        match manager.get_note(&note.id) {
            Err(NFAError::NoteNotFound) => assert!(true),
            _ => assert!(false, "Note should have been deleted"),
        }
    }

    #[test]
    fn test_list_notes() {
        let (manager, _dir) = setup();

        let note1 = manager
            .create_note("Note 1".to_string(), "Content 1".to_string())
            .unwrap();
        let note2 = manager
            .create_note("Note 2".to_string(), "Content 2".to_string())
            .unwrap();

        let notes = manager.list_notes().unwrap();

        assert_eq!(notes.len(), 2);
        assert!(notes.contains(&note1));
        assert!(notes.contains(&note2));
    }
}
