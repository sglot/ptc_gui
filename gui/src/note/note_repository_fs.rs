pub mod note_repository_fs {
    use crate::config::config::Config;
    use crate::note::note::note::Note;
    use crate::note::note_repository::note_repository::NoteRepository;
    use crate::registry::registry::Registry;
    use crate::support::fs::maker::maker_fs::MakerFS;
    use std::fs;
    use std::io::empty;
    use std::path::Path;
    use std::fs::OpenOptions;

    pub struct NoteRepositoryFS {
        config: Config,
        maker: MakerFS,
    }

    impl NoteRepositoryFS {
        fn append_to_file(
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

            let mut file_ref = match OpenOptions::new().append(true).open(&path) {
                Ok(file) => file,
                Err(_) => match File::create(&path) {
                    Ok(it) => it,
                    Err(err) => return Err(err.to_string()),
                },
            };   
    
            
            match file_ref.write_all(data.as_bytes()) {
                Ok(_) => Ok("Заметка успешно сохранена".to_string()),
                Err(e) => return Err(e.to_string()),
            }
        }
    }

    impl NoteRepository for NoteRepositoryFS {
        fn save(&self, note: Note) -> Result<String, String> {
            
            let path = self
                .maker
                .make_user_notebook_dir(&note.user());

            let mut data = serde_json::to_string(&note).unwrap();
            data.push_str("@,@");

            let res = match self.append_to_file(
                        &path,
                        &data,
                        "2023",
                    ) {
                        Ok(res) => Ok(res),
                        Err(e) => Err(e),
                    };

            res
        }

        // fn update(&self, resource: Resource) -> Result<String, String> {
        //     let path = self
        //         .maker
        //         .make_template_dir(&resource.user_login(), &resource.name());

        //     let res = match Path::new(&path).exists() {
        //         false => Err(String::from("Ошибка, такого ресурса не существует")),
        //         true => {
        //             let cryptor = Cryptor { key: self.config.get_secret_key() };

        //             let template_password_crypted;

        //             // проверка на зашифрованный пароль, чтобы не перешифровать его ещё раз
        //             match cryptor.decrypt(&resource.template_password()) {
        //                 Ok(_) => template_password_crypted = resource.template_password(),
        //                 Err(_) => template_password_crypted = cryptor.crypt(&resource.template_password()),
        //             };

        //             let mut resource_data:Vec<String> = Vec::new();
        //             resource_data.push(template_password_crypted);
        //             resource_data.push(resource.resource_login());
        //             // resource_data.push(resource.master_password());

        //             match self.save_to_file(
        //                 &path,
        //                 &resource_data.join(";"),
        //                 self.config.get_template_file_name(),
        //             ) {
        //                 Ok(res) => Ok(res),
        //                 Err(e) => Err(e),
        //             }
        //         }
        //     };

        //     res
        // }

        // fn delete(&self, resource: Resource) -> Result<String, String> {
        //     let path = self
        //         .maker
        //         .make_template_dir(&resource.user_login(), &resource.name());

        //     match Path::new(&path).exists() {
        //         true => match fs::remove_dir_all(&path) {
        //             Ok(_) => Ok("Ресурс удалён".to_string()),
        //             Err(e) => Err(e.to_string()),
        //         },
        //         false => Ok("Ресурс удалён".to_string()),
        //     }
        // }

        fn get_list(&self, login: &str) -> Result<Vec<Note>, String> {
            let notebook_path = self.maker.make_user_notebook_dir(login);
            tracing::error!("{notebook_path}");
            let paths = match fs::read_dir(notebook_path) {
                Ok(line) => line,
                Err(e) => return Err(e.to_string()),
            };

            let mut note_files = vec![];
            for path in paths {

                note_files.push(
                    path.unwrap()
                        .path(),
                );

            }
            tracing::error!("{note_files:?}");

            let mut notes = Vec::new();
            for file in note_files {
                let data = match fs::read_to_string(file) {
                    Ok(val) => val,
                    Err(_err) => return Err(String::from("не найден указанный ресурс")),
                };

                tracing::error!("{data}");
                // let mut json = "[".to_string();
                // json.push_str(&data);
                // json.push_str("]");
                

                let data: Vec<&str> = data.split("@,@").collect();
                for note_string in data {
                    tracing::error!("{note_string:?}");
                    if note_string.is_empty() {
                        continue;
                    }

                    let note: Note = serde_json::from_str(&note_string).unwrap();
                    notes.push(note);
                }

            }

            tracing::error!("{notes:?}");

            Ok(notes)
        }

        // fn find(&self, user_login: &str, resource: &str) -> Result<Resource, String> {
        //     // TODO: 

        //     // if !auth.check_auth(&login) {
        //     //     return String::from("Недостаточно прав");
        //     // }

        //     let path_to_resource_data = self.maker.make_template_path(user_login, &resource);

        //     let resource_data = match fs::read_to_string(path_to_resource_data) {
        //         Ok(val) => val,
        //         Err(_err) => return Err(String::from("не найден указанный ресурс")),
        //     };

        //     let resource_data: Vec<&str> = resource_data.split(";").collect();
        //     let template_password = resource_data[0].to_string();
        //     let login = resource_data[1].to_string();
        //     // let master_password = resource_data[2].to_string();
        //     Ok(Resource::new(
        //         template_password,
        //         login,
        //         // master_password,
        //         resource.to_string(),
        //         user_login.to_string(),
        //     ))
        // }
    }
    impl NoteRepositoryFS {
        pub fn new() -> NoteRepositoryFS {
            let config = Registry::config();
            let maker = MakerFS::new(config.clone());

            NoteRepositoryFS {
                config: config,
                maker,
            }
        }
    }
}
