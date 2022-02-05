pub mod resource_add_service {
    use crate::resource::{resource_repository_fs::resource_repository_fs::ResourceRepositoryFS, resource_repository::resource_repository::ResourceRepository};


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
        
        pub fn resource_add(&self, login: &str) -> Result<String, String> {

            

            match self.resource_repository.get_list(&login) {
                Ok(res) => Ok(res),
                Err(e) => Err(e),
            }
            
        }

    }

    
}
