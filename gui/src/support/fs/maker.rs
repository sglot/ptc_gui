pub mod maker_fs {
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
            let mut path = String::new();
            let storage_file_name: &str = self.config.get_user_password_file_name();

            path.push_str(self.config.get_root_path());
            path.push_str("./.ptc/");
            path.push_str(login);
            path.push_str(storage_file_name);

            path
        }

        pub fn make_template_path(&self, login: &str, resource: &str) -> String {
            let mut path = String::new();
            let storage_file_name: &str = self.config.get_template_file_name();

            path.push_str(self.config.get_root_path());
            path.push_str("./.ptc/");
            path.push_str(login);
            path.push_str("/templates/");
            path.push_str(resource);
            path.push_str(storage_file_name);

            path
        }

        pub fn make_template_dir(&self, login: &str, resource: &str) -> String {
            let mut path = String::new();
            path.push_str(self.config.get_root_path());
            path.push_str("./.ptc/");
            path.push_str(login);
            path.push_str("/templates/");
            path.push_str(resource);
            path
        }

        pub fn make_user_templates_dir(&self, login: &str) -> String {
            let mut path = String::new();
            path.push_str(self.config.get_root_path());
            path.push_str("./.ptc/");
            path.push_str(login);
            path.push_str("/templates/");
            path
        }

        pub fn make_user_home_dir(&self, login: &str) -> String {
            let mut path = String::new();
            path.push_str(self.config.get_root_path());
            path.push_str(".//.ptc//");
            path.push_str(login);

            path
        }
    }
}
