pub mod note_list_service {
    use crate::{
        note::{
            note::note::Note, note_repository::note_repository::NoteRepository,
            note_repository_fs::note_repository_fs::NoteRepositoryFS,
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
            tracing::error!("load notes");
            let changed_note_list = REGISTRY
                .lock()
                .unwrap()
                .form_data
                .note_list
                .typed_cache
                .note_list();
            tracing::error!("load notes1");
            if !changed_note_list.is_empty() {
                tracing::error!("load notes2");
                return Ok(changed_note_list);
            }
            tracing::error!("load notes3");
            match self.note_repository.get_list(&login) {
                Ok(res) => {
                    tracing::error!("load notes4");
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

        pub fn load_for_update_from_list(&self, note: &Note) {
            REGISTRY.lock().unwrap().form_data.note_add.id = note.id();
            REGISTRY.lock().unwrap().form_data.note_add.date = note.date();
            REGISTRY.lock().unwrap().form_data.note_add.cost = note.cost.unwrap().to_string();
            REGISTRY.lock().unwrap().form_data.note_add.mileage = note.mileage.unwrap().to_string();
            REGISTRY.lock().unwrap().form_data.note_add.text = note.text();
            REGISTRY.lock().unwrap().form_data.note_add.title = note.title();
            REGISTRY.lock().unwrap().form_data.note_add.tags = note.tags();
        }

        pub fn delete(&self, note: u64) -> Result<String, String> {
            tracing::error!("1111111111111111111111111111");
            let res = self
                .note_repository
                .delete(note, &REGISTRY.lock().unwrap().auth_data.login.clone());

            if res.is_err() {
                ()
            }

            // чтобы при удалении и сбросе typed_cache не было блокирования
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .note_list
                .typed_cache
                .drop_note_list();
            
            Ok("Успешно".to_string())
        }
    }
}
