pub mod resource_add_service {
    use crate::resource::{resource_repository_fs::resource_repository_fs::ResourceRepositoryFS, resource_repository::resource_repository::ResourceRepository, resource::resource::Resource};


    pub struct ResourceAddService {
        // cryptor: Box<Cryptor>,
        // maker: MakerFS,
        resource_repository: ResourceRepositoryFS
    }

    impl ResourceAddService {
        pub fn new() -> ResourceAddService {
            // let cryptor = Cryptor { key: Registry::config().get_secret_key() };
            // let maker = MakerFS::new(Registry::config());
            // let auth = AuthFs::new(cryptor.clone(), maker.clone());

            ResourceAddService {
                // cryptor: Box::new(cryptor),
                // maker: maker,
                resource_repository: ResourceRepositoryFS::new()
            }
        }
        
        pub fn resource_add(&self, resource: Resource) -> Result<String, String> {
            tracing::error!("resource_add");
            match self.resource_repository.save(resource) {
                Ok(res) => Ok(res),
                Err(e) => Err(e),
            }
            
        }

    }

    
}
