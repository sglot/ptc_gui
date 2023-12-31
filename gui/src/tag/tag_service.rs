pub mod tag_service {
    use crate::{
         tag::tag_repository::tag_repository::TagRepository, 
         tag::tag_repository_fs::tag_repository_fs::TagRepositoryFS, notebook::add::note_add_form_facade::note_add_form_facade::NoteAddFormFacade, 
    };

    
    pub struct TagService {
        tag_repository: TagRepositoryFS
    }

    impl TagService {
        pub fn new() -> TagService {
            TagService {
                tag_repository: TagRepositoryFS::new()
            }
        }
        
        pub fn add(&self, tag: String) -> Result<String, String> {
            match self.tag_repository.find(tag.clone()) {
                Ok(_) => {Err("Тэг уже существует".to_string())},
                Err(_) => {
                    match self.tag_repository.save(tag.clone()) {
                        Ok(res) => {
                            tracing::error!("3");
                            NoteAddFormFacade::push_to_tag_list(tag.clone());
                            // REGISTRY.lock().unwrap().form_data.note_add.tag_list.push(tag.clone());
                            tracing::error!("4");
                            Ok(res)
                        },
                        Err(e) => Err(e),
                    } 
                }
            }    
        }

        pub fn delete(&self, tag: String) -> Result<String, String> {
            match self.tag_repository.delete(tag.clone()) {
                Ok(res) => {
                    tracing::error!("3");
                    NoteAddFormFacade::set_tag_list(res);
                    // REGISTRY.lock().unwrap().form_data.note_add.tag_list.push(tag.clone());
                    tracing::error!("4");
                    Ok("Успешно".to_string())
                },
                Err(e) => Err(e),
            } 

        }
    }

    
}
