pub mod tag_repository_fs {
    // use crate::config::config::Config;
    use crate::registry::registry::Registry;
    use crate::support::fs::fs::DELIMITER;
    use crate::support::fs::maker::maker_fs::MakerFS;
    use crate::tag::tag_repository::tag_repository::TagRepository;
    use std::fs::{self, OpenOptions};

    pub struct TagRepositoryFS {
        // config: Config,
        maker: MakerFS,
    }

    impl TagRepositoryFS {
        fn append_to_file(
            &self,
            path_to: &str,
            data: &str,
            storage_file_name: &str,
        ) -> Result<String, String> {
            use std::fs::File;
            use std::io::Write;
            tracing::error!("8");
            let mut path = String::from(path_to);
            path.push_str(storage_file_name);
            let mut file_ref = match OpenOptions::new().append(true).open(&path) {
                Ok(file) => file,
                Err(_) => match File::create(&path) {
                    Ok(it) => it,
                    Err(err) => return Err(err.to_string()),
                },
            };   
    
            tracing::error!("7");
            match file_ref.write_all(data.as_bytes()) {
                Ok(_) => Ok("Успешно".to_string()),
                Err(e) => return Err(e.to_string()),
            }
        }
    }

    impl TagRepository for TagRepositoryFS {
        fn get_list(&self) -> Result<Vec<String>, String> {
            let tags_path = self.maker.make_db_path("tags".to_string());
            tracing::error!("{tags_path}");
                let data = match fs::read_to_string(tags_path) {
                    Ok(val) => val,
                    Err(_err) => return Err(String::from("не найден указанный ресурс")),
                };

                tracing::error!("{data}");

                let tags = data.split(DELIMITER);

            tracing::error!("{tags:?}");
            
            Ok(tags.map(String::from).collect())
        }

        fn save(&self, tag: String) -> Result<String, String> {
            let path = self
                .maker
                .make_db_path("tags".to_string());

            let mut data = "@,@".to_string();
            data.push_str(&tag);
            tracing::error!("5");
            let res = match self.append_to_file(
                        &path,
                        &data,
                        "",
                    ) {
                        Ok(res) => Ok(res),
                        Err(e) => Err(e),
                    };

            res
        }

        fn delete(&self, tag: String) -> Result<Vec<String>, String> {
            let tags_res = self.get_list();
            let mut tags;
            match tags_res {
                Ok(tags_vec) => {
                    use std::fs::File;
                    use std::io::Write;

                    let index = tags_vec.iter().position(| x| x.eq(&tag)).unwrap().clone();
                    tags = tags_vec;
                    tags.remove(index);
                    tracing::error!("------------");

                    let mut data = "".to_string();
                    for tag in &tags {
                        if !data.is_empty() {
                            data.push_str("@,@")
                        };
                        data.push_str(&tag);
                    }

                    let path = self
                                        .maker
                                        .make_db_path("tags".to_string());

                    let mut file_ref = match OpenOptions::new().open(&path) {
                        Ok(file) => file,
                        Err(_) => match File::create(&path) {
                            Ok(it) => it,
                            Err(err) => return Err(err.to_string()),
                        },
                    };   
            
                    match file_ref.write_all(data.as_bytes()) {
                        Ok(_) => Ok(tags.clone()),
                        Err(e) => return Err(e.to_string()),
                    }

                   
                },
                Err(e) => return Err(e.to_string()),
            }

        }

        fn find(&self, tag: String) -> Result<String, String> {
            let tags_res = self.get_list();

            match tags_res {
                Ok(tags_vec) => {
                    match tags_vec.iter().find(| x| x.eq(&&tag)) {
                        None => {tracing::error!("--------None"); Err(String::new())},
                        Some(str) => {tracing::error!("--------str--"); Ok(str.clone())},
                    }                  
                },
                Err(e) => return Err(e.to_string()),
            }
        }
    }
    impl TagRepositoryFS {
        pub fn new() -> TagRepositoryFS {
            let config = Registry::config();
            let maker = MakerFS::new(config.clone());

            TagRepositoryFS {
                // config: config,
                maker,
            }
        }
    }
}
