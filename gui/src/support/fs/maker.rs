pub mod maker_fs {
    use chrono::Utc;

    use crate::config::config::Config;


    #[derive(Debug, Clone)]
    pub struct MakerFS {
        config: Config,
    }

    impl MakerFS {
        pub fn new(config: Config) -> MakerFS {
            MakerFS { config: config }
        }

        pub fn make_user_path(&self, login: &str) -> String {
            let storage_file_name: &str = self.config.get_user_password_file_name();

            let mut path = self.make_user_home_dir(login);
            path.push_str(storage_file_name);

            path
        }

        pub fn make_template_path(&self, login: &str, resource: &str) -> String {
            let storage_file_name: &str = self.config.get_template_file_name();

            let mut path = self.make_template_dir(login, resource);
            path.push_str(storage_file_name);

            path
        }

        pub fn make_template_dir(&self, login: &str, resource: &str) -> String {
            let mut path = self.make_user_templates_dir(login);
            path.push_str(resource);

            path
        }

        pub fn make_user_templates_dir(&self, login: &str) -> String {
            let mut path = self.make_user_home_dir(login);
            path.push_str("/templates/");

            path
        }

        pub fn make_user_home_dir(&self, login: &str) -> String {
            let mut path = String::new();
            path.push_str(self.config.get_root_path());
            path.push_str("./.ptc/users/");
            path.push_str(login);

            path
        }

        // notebook
        pub fn make_user_notebook_dir(&self, login: &str) -> String {
            let mut path = self.make_user_home_dir(login);
            path.push_str("/notebook/");
            
            path
        }

        // service
        pub fn make_db_path(&self, entyty: String) -> String {
            format!("{}/{}.db", self.make_program_data_path(), entyty)
        }

        pub fn make_program_data_path(&self) -> String {
            format!("{}./.ptc/program_data", self.config.get_root_path())
        }

        pub fn delimiter(&self) -> &str {
            "@,@"
        }

        pub fn year(&self) -> String {
            Utc::now().format("%Y").to_string()
        }
    }
}
