pub mod note_list_service {
    use crate::{
        note::{
            note::note::Note, 
            note_repository_fs::note_repository_fs::NoteRepositoryFS,
            note_repository::note_repository::NoteRepository
        },
        REGISTRY,
    };

    pub struct NoteListService {
        note_repository: NoteRepositoryFS,
    }

    impl NoteListService {
        pub fn new() -> NoteListService {
            NoteListService {
                note_repository: NoteRepositoryFS::new(),
            }
        }

        pub fn note_list(&self, login: &str) -> Result<Vec<Note>, String> {
            // требуется обновление кэша
            // 1. delete res
            // 2. add res

            let changed_note_list = REGISTRY
                .lock()
                .unwrap()
                .form_data
                .note_list
                .typed_cache
                .note_list();
            if !changed_note_list.is_empty() {
                return Ok(changed_note_list);
            }

            match self.note_repository.get_list(&login) {
                Ok(res) => {
                    REGISTRY
                        .lock()
                        .unwrap()
                        .form_data
                        .note_list
                        .typed_cache
                        .set_note_list(res.clone());
                    Ok(res)
                }
                Err(e) => Err(e),
            }
        }
    }
}
