pub mod resource_add_service {
    use crate::{resource::{resource_repository_fs::resource_repository_fs::ResourceRepositoryFS, resource_repository::resource_repository::ResourceRepository, resource::resource::Resource}, REGISTRY};


    pub struct ResourceAddService {
        resource_repository: ResourceRepositoryFS
    }

    impl ResourceAddService {
        pub fn new() -> ResourceAddService {
            ResourceAddService {
                resource_repository: ResourceRepositoryFS::new()
            }
        }
        
        pub fn resource_add(&self, resource: Resource) -> Result<String, String> {
            match self.resource_repository.save(resource) {
                Ok(res) => {
                    REGISTRY.lock().unwrap().form_data.resource_list.typed_cache.drop_resource_list();
                    Ok(res)
                },
                Err(e) => Err(e),
            } 
        }

        pub fn resource_update(&self, resource: Resource) -> Result<String, String> {
            match self.resource_repository.update(resource) {
                Ok(res) => {
                    REGISTRY.lock().unwrap().form_data.resource_list.typed_cache.drop_resource_list();
                    Ok(res)
                },
                Err(e) => Err(e),
            } 
        }

    }

    
}
