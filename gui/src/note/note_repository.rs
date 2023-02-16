pub mod note_repository{
    use crate::note::note::note::Note;

    

    pub trait NoteRepository {
        fn save(&self, note: Note) -> Result<String, String>;
        // fn update(&self, note: Resource) -> Result<String, String>;
        // fn delete(&self, note: Resource) -> Result<String, String>;
        fn get_list(&self, login: &str) -> Result<Vec<Note>, String>;
        // fn find(&self, login: &str, resource: &str) -> Result<Resource, String>;
    }
}