pub mod resource_repository{
    use crate::resource::resource::resource::Resource;

    pub trait ResourceRepository {
        fn save(&self, resource: Resource) -> Result<String, String>;
        fn get_list(&self, login: &str) -> Result<Vec<String>, String>;
        fn find(&self, login: &str, resource: &str) -> Result<Resource, String>;
    }
}