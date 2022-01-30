pub mod resource_list_service {
    use crate::resource::{resource_repository_fs::resource_repository_fs::ResourceRepositoryFS, resource_repository::resource_repository::ResourceRepository};


    pub struct ResourceListService {
        // cryptor: Box<Cryptor>,
        // maker: MakerFS,
        resource_repository: ResourceRepositoryFS
    }

    impl ResourceListService {
        pub fn new() -> ResourceListService {
            // let cryptor = Cryptor { key: Registry::config().get_secret_key() };
            // let maker = MakerFS::new(Registry::config());
            // let auth = AuthFs::new(cryptor.clone(), maker.clone());

            ResourceListService {
                // cryptor: Box::new(cryptor),
                // maker: maker,
                resource_repository: ResourceRepositoryFS::new()
            }
        }
        
        pub fn resource_list(&self, login: &str) -> Result<String, String> {

            

            match self.resource_repository.get_list(&login) {
                Ok(res) => Ok(res),
                Err(e) => Err(e),
            }
            
        }

    }

    
}
