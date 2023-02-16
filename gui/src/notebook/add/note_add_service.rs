pub mod note_add_service {
    use crate::{
        REGISTRY, 
        note::{
            note_repository_fs::note_repository_fs::NoteRepositoryFS,
             note_repository::note_repository::NoteRepository,
             note::note::Note}
            };

    
    pub struct NoteAddService {
        note_repository: NoteRepositoryFS
    }

    impl NoteAddService {
        pub fn new() -> NoteAddService {
            NoteAddService {
                note_repository: NoteRepositoryFS::new()
            }
        }
        
        pub fn add(&self, note: Note) -> Result<String, String> {
            let s = self.note_repository.get_list("t");
        
            match self.note_repository.save(note) {
                Ok(res) => {
                    REGISTRY.lock().unwrap().form_data.note_list.typed_cache.drop_note_list();
                    Ok(res)
                },
                Err(e) => Err(e),
            } 

            }

        // pub fn update(&self, note: Note) -> Result<String, String> {
        //     match self.note_repository.update(note) {
        //         Ok(res) => {
        //             // REGISTRY.lock().unwrap().form_data.resource_list.typed_cache.drop_resource_list();
        //             Ok(res)
        //         },
        //         Err(e) => Err(e),
        //     } 
        // }

    }

    
}
