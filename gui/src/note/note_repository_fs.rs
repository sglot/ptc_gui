pub mod note_repository_fs {
    // use crate::config::config::Config;
    use crate::note::note::note::Note;
    use crate::note::note_repository::note_repository::NoteRepository;
    use crate::registry::registry::Registry;
    use crate::support::fs::maker::maker_fs::MakerFS;
    use std::fs;
    use std::fs::OpenOptions;

    pub struct NoteRepositoryFS {
        // config: Config,
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

        fn save_all_notes(&self, data_for_save: String, user: &str) -> Result<String, String> {
            let path = self
                .maker
                .make_user_notebook_dir(user);

            let year = self.maker.year();
            let year_duplicate = self.maker.year() + "_duplicate";

            let file_name_orig = path.clone() + &year;
            let file_name_duplicate = path.clone() + &year_duplicate;
            
            let res: _ = match self.append_to_file(
                        &path,
                        &data_for_save,
                        &year_duplicate,
                    ) {
                        Ok(res) => {
                            //remove and rename
                            fs::remove_file(&file_name_orig);
                            fs::rename(file_name_duplicate, &file_name_orig);
                            tracing::error!("000000000000000000k");
                            Ok(res)
                        },
                        Err(e) => Err(e),
                    };
            res
        }
    }

    impl NoteRepository for NoteRepositoryFS {
        fn save(&self, note: Note) -> Result<String, String> {
            
            let path = self
                .maker
                .make_user_notebook_dir(&note.user());

            let mut data = serde_json::to_string(&note).unwrap();
            data.push_str(self.maker.delimiter());

            let res = match self.append_to_file(
                        &path,
                        &data,
                        &self.maker.year(),
                    ) {
                        Ok(res) => Ok(res),
                        Err(e) => Err(e),
                    };

            res
        }

        fn update(&self, note: Note) -> Result<String, String> {
            let mut all_notes = self.get_list(&note.user).unwrap();

            let exist = match all_notes.iter().position(| x| x.id() == note.id()) {
                None => {tracing::error!("id уже существует"); Err("id не существует".to_string())},
                Some(position) => {
                    all_notes.remove(position);
                    all_notes.insert(position, note.clone());
                    Ok(())
                },
            }; 
            // tracing::error!("{all_notes:?}");
            // std::process::exit(0);
            if exist.is_err() {
                exist.unwrap()
            }

            let mut data_for_save = "".to_string();
            for note in all_notes {
                data_for_save.push_str(&serde_json::to_string(&note).unwrap());
                data_for_save.push_str(self.maker.delimiter());
            }

            self.save_all_notes(data_for_save, &note.user())
        }

        fn delete(&self, note_id: u64, user: &str) -> Result<String, String> {
            let mut all_notes = self.get_list(user).unwrap();

            let exist = match all_notes.iter().position(| x| x.id() == note_id) {
                None => {tracing::error!("id уже существует"); Err("id не существует".to_string())},
                Some(position) => {
                    all_notes.remove(position);
                    Ok(())
                },
            }; 
            tracing::error!("/////////////////1111111");
            // // tracing::error!("{note:?}");
            // std::process::exit(0);
            if exist.is_err() {
                tracing::error!("/////////////////2222222");
                exist.unwrap()
            }

            tracing::error!("/////////////////3333333");
            let mut data_for_save = "".to_string();
            for note in all_notes {
                data_for_save.push_str(&serde_json::to_string(&note).unwrap());
                data_for_save.push_str(self.maker.delimiter());
            }

            self.save_all_notes(data_for_save, user)
        }

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
                // config: config,
                maker,
            }
        }
    }
}
