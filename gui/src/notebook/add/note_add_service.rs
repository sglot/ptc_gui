pub mod note_add_service {
    use crate::{
        note::{
            note::note::Note, note_repository::note_repository::NoteRepository,
            note_repository_fs::note_repository_fs::NoteRepositoryFS,
        },
        REGISTRY,
    };

    pub struct NoteAddService {
        note_repository: NoteRepositoryFS,
    }

    impl NoteAddService {
        pub fn new() -> NoteAddService {
            NoteAddService {
                note_repository: NoteRepositoryFS::new(),
            }
        }

        pub fn add(&self, note: Note) -> Result<String, String> {
            let login = REGISTRY.lock().unwrap().auth_data.login.clone();
            let all_notes = self.note_repository.get_list(&login).unwrap();

            match all_notes.iter().find(| x| x.id() == note.id()) {
                Some(_) => {tracing::error!("id уже существует"); Err("id уже существует".to_string())},
                None => match self.note_repository.save(note) {
                    Ok(res) => {
                        REGISTRY
                            .lock()
                            .unwrap()
                            .form_data
                            .note_list
                            .typed_cache
                            .drop_note_list();

                            REGISTRY
                            .lock()
                            .unwrap()
                            .form_data
                            .note_summary
                            .typed_cache
                            .drop();
                        Ok(res)
                    }
                    Err(e) => Err(e),
                },
            } 
        }

        pub fn update(&self, note: Note) -> Result<String, String> {
            match self.note_repository.update(note) {
                    Ok(res) => {
                        REGISTRY
                            .lock()
                            .unwrap()
                            .form_data
                            .note_list
                            .typed_cache
                            .drop_note_list();

                            REGISTRY
                            .lock()
                            .unwrap()
                            .form_data
                            .note_summary
                            .typed_cache
                            .drop();
                        Ok(res)
                    }
                    Err(e) => Err(e),
            }
        }
    }
}
