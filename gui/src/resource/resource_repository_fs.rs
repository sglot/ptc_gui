pub mod resource_repository_fs {
    use crate::config::config::Config;
    use crate::registry::registry::Registry;
    use crate::resource::resource_repository::resource_repository::ResourceRepository;
    use crate::support::fs::maker::maker_fs::MakerFS;
    use crate::user::user::user::User;
    use std::fs;

    pub struct ResourceRepositoryFS {
        config: Config,
        maker: MakerFS,
    }

    impl ResourceRepository for ResourceRepositoryFS {
        fn save() {
            todo!()
        }

        fn get_list(&self, login: &str) -> Result<String, String> {
            let templates_path = self.maker.make_user_templates_dir(login);

            let paths = match fs::read_dir(templates_path) {
                Ok(line) => line,
                Err(e) => return Err(e.to_string()),
            };

            for path in paths {
                println!(
                    "Name: {}",
                    path.unwrap()
                        .path()
                        .into_iter()
                        .last()
                        .unwrap()
                        .to_str()
                        .unwrap()
                )
            }

            Ok("f".to_string())
        }
    }
    impl ResourceRepositoryFS {
        pub fn new() -> ResourceRepositoryFS {
            let config = Registry::config();
            let maker = MakerFS::new(config.clone());
            // let maker = MakerFS::new(config.clone());
            ResourceRepositoryFS { config: config, maker }
        }
    }
}
