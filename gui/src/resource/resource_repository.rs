pub mod resource_repository{
    pub trait ResourceRepository {
        fn save();
        fn get_list(&self, login: &str) -> Result<String, String>;
    }
}