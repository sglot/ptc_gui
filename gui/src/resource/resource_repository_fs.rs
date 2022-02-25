pub mod resource_repository_fs {
    use crate::config::config::Config;
    use crate::cryptor::cryptor::Cryptor;
    use crate::registry::registry::Registry;
    use crate::resource::resource::resource::Resource;
    use crate::resource::resource_repository::resource_repository::ResourceRepository;
    use crate::support::fs::maker::maker_fs::MakerFS;
    use std::fs;
    use std::path::Path;

    pub struct ResourceRepositoryFS {
        config: Config,
        maker: MakerFS,
    }

    impl ResourceRepositoryFS {
        fn save_to_file(
            &self,
            path_to: &str,
            data: &str,
            storage_file_name: &str,
        ) -> Result<String, String> {
            use std::fs::File;
            use std::io::Write;

            let mut path = String::from(path_to);
            match fs::create_dir_all(&path) {
                Ok(it) => it,
                Err(err) => return Err(err.to_string()),
            };

            path.push_str(storage_file_name);
            let mut output = match File::create(&path) {
                Ok(it) => it,
                Err(err) => return Err(err.to_string()),
            };
            tracing::error!("save_to_file write");
            match write!(output, "{}", data) {
                Ok(_) => Ok("Шаблон успешно сохранён".to_string()),
                Err(e) => return Err(e.to_string()),
            }
        }
    }

    impl ResourceRepository for ResourceRepositoryFS {
        fn save(&self, resource: Resource) -> Result<String, String> {
            let path = self
                .maker
                .make_template_dir(&resource.login(), &resource.name());

            let res = match Path::new(&path).exists() {
                true => Err(String::from("Такой ресурс уже существует")),
                false => {
                    let cryptor = Cryptor { key: self.config.get_secret_key() };
                    tracing::error!("save_savee");
                    match self.save_to_file(
                        &path,
                        &cryptor.crypt(&resource.template()),
                        self.config.get_template_file_name(),
                    ) {
                        Ok(res) => Ok(res),
                        Err(e) => Err(e),
                    }
                }
            };

            res
        }

        fn delete(&self, resource: Resource) -> Result<String, String> {
            let path = self
                .maker
                .make_template_dir(&resource.login(), &resource.name());

            match Path::new(&path).exists() {
                true => match fs::remove_dir_all(&path) {
                    Ok(_) => Ok("Ресурс удалён".to_string()),
                    Err(e) => Err(e.to_string()),
                },
                false => Ok("Ресурс удалён".to_string()),
            }
        }

        fn get_list(&self, login: &str) -> Result<Vec<String>, String> {
            let templates_path = self.maker.make_user_templates_dir(login);

            let paths = match fs::read_dir(templates_path) {
                Ok(line) => line,
                Err(e) => return Err(e.to_string()),
            };

            let mut resources = vec![];
            for path in paths {
                resources.push(
                    path.unwrap()
                        .path()
                        .into_iter()
                        .last()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                );
            }

            Ok(resources)
        }

        fn find(&self, login: &str, resource: &str) -> Result<Resource, String> {
            // if !auth.check_auth(&login) {
            //     return String::from("Недостаточно прав");
            // }

            let template_path = self.maker.make_template_path(login, &resource);

            let template = match fs::read_to_string(template_path) {
                Ok(val) => val,
                Err(_err) => return Err(String::from("не найден указанный ресурс")),
            };

            Ok(Resource::new(
                resource.to_string(),
                template,
                login.to_string(),
            ))
        }
    }
    impl ResourceRepositoryFS {
        pub fn new() -> ResourceRepositoryFS {
            let config = Registry::config();
            let maker = MakerFS::new(config.clone());

            ResourceRepositoryFS {
                config: config,
                maker,
            }
        }
    }
}
