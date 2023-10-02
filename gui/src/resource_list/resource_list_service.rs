pub mod resource_list_service {
    use crate::{resource::{resource_repository_fs::resource_repository_fs::ResourceRepositoryFS, resource_repository::resource_repository::ResourceRepository, resource::resource::Resource}, cryptor::cryptor::Cryptor, registry::registry::Registry, REGISTRY};


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
            // требуется обновление кэша
            // 1. delete res
            // 2. add res

            let changed_resource_list = REGISTRY.lock().unwrap().form_data.resource_list.typed_cache.resource_list();
            if !changed_resource_list.is_empty() {
                return Ok(changed_resource_list)
            }

            match self.resource_repository.get_list(&login) {
                Ok(res) => {
                    REGISTRY.lock().unwrap().form_data.resource_list.typed_cache.set_resource_list(res.clone());
                    Ok(res)
                },
                Err(e) => Err(e),
            }
        }

        pub fn find_template_pass(&self, login: &str, resource_name: &str) -> Result<String, String> {
            match self.resource_repository.find(&login, resource_name) {
                Ok(resource) => Ok(resource.template_password()),
                Err(e) => Err(e),
            }
        }

        pub fn find_resource(&self, login: &str, resource_name: &str) -> Result<Resource, String> {
            match self.resource_repository.find(&login, resource_name) {
                Ok(resource) => Ok(resource),
                Err(e) => Err(e),
            }
        }
        
        pub fn decrypt_template_pass(&self, template: &str) -> Result<String, String> {
            let cryptor = Cryptor { key: Registry::config().get_secret_key() };

            cryptor.decrypt(template)
        }

        pub fn resource_delete(&self, resource: Resource) -> Result<String, String> {
            match self.resource_repository.delete(resource) {
                Ok(res) => {
                    REGISTRY.lock().unwrap().form_data.resource_list.typed_cache.drop_resource_list();
                    Ok(res)
                },
                Err(e) => Err(e),
            }
            
        }

    }

    
}
