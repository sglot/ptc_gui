pub mod resource_list_service {
    use crate::{resource::{resource_repository_fs::resource_repository_fs::ResourceRepositoryFS, resource_repository::resource_repository::ResourceRepository, self, resource::resource::Resource}, cryptor::cryptor::Cryptor, registry::registry::Registry};


    pub struct ResourceListService {
        resource_repository: ResourceRepositoryFS
    }

    impl ResourceListService {
        pub fn new() -> ResourceListService {
            ResourceListService {
                resource_repository: ResourceRepositoryFS::new()
            }
        }
        
        pub fn resource_list(&self, login: &str) -> Result<Vec<String>, String> {
            match self.resource_repository.get_list(&login) {
                Ok(res) => Ok(res),
                Err(e) => Err(e),
            }
        }

        pub fn find_template(&self, login: &str, resource_name: &str) -> Result<String, String> {
            match self.resource_repository.find(&login, resource_name) {
                Ok(resource) => Ok(resource.template()),
                Err(e) => Err(e),
            }
        }
        
        pub fn decrypt_template(&self, template: &str) -> Result<String, String> {
            let cryptor = Cryptor { key: Registry::config().get_secret_key() };

            cryptor.decrypt(template)
        }

        pub fn resource_delete(&self, resource: Resource) -> Result<String, String> {
            tracing::error!("resource_delete");
            match self.resource_repository.delete(resource) {
                Ok(res) => Ok(res),
                Err(e) => Err(e),
            }
            
        }

    }

    
}
