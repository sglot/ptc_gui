pub mod bootstrap {
    use std::path::Path;

    use crate::{
        support::fs::{save::save_fs, maker::maker_fs::MakerFS}, 
        config::config::Config, REGISTRY, tag::tag_repository_fs::tag_repository_fs::TagRepositoryFS, tag::tag_repository::tag_repository::TagRepository,
    };

    pub fn init(config: Config) -> () {
        let maker = MakerFS::new(config);

        let program_data_path = maker.make_program_data_path();
        let tags_path = maker.make_db_path("tags".to_string());

        let res = match Path::new(&tags_path).exists() {
            true => Err(String::from("Теги уже записаны")),
            false => {
                match save_fs::save_to_file(&program_data_path, "машина@,@ кашкай@,@ пробег@,@ расходники@,@ ТО@,@ сервис@,@ бензин@,@ 2022@,@ 2023", "./tags.db") {
                    Ok(_) => Ok("Теги записаны".to_string()),
                    Err(e) => Err(e.to_string()),
                }
            }
        };
        tracing::error!("  {:?}", res);



        let tag_repository: TagRepositoryFS = TagRepositoryFS::new();
            REGISTRY.lock().unwrap().form_data.note_add.tag_list = match tag_repository.get_list() {
                Ok(tags) => tags,
                Err(_) => vec![],
            };

    }
}
