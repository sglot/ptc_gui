pub mod tag_repository{
    pub trait TagRepository {
        fn save(&self, tag: String) -> Result<String, String>;
        // fn update(&self, note: Resource) -> Result<String, String>;
        fn delete(&self, tag: String) -> Result<Vec<String>, String>;
        fn get_list(&self) -> Result<Vec<String>, String>;
        fn find(&self, tag: String) -> Result<String, String>;
    }
}