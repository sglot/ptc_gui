pub mod config {
    #[derive(Debug, Clone)]
    pub struct Config {
        root_path: String,
        secret_key: String,
        template_file_name: String,
        user_password_file_name: String,
    }

    impl Config {
        pub fn new() -> Config {
            // начинать будем с текущей директории
            Config {
                root_path: String::from("./"),
                secret_key: String::from("standard"),
                template_file_name: String::from("/resource_data.sec"),
                user_password_file_name: String::from("/storage.sec"),
            }
        }

        pub fn get_root_path(&self) -> &str {
            &self.root_path
        }

        pub fn get_secret_key(&self) -> String {
            self.secret_key.clone()
        }

        pub fn get_template_file_name(&self) -> &str {
            &self.template_file_name
        }

        pub fn get_user_password_file_name(&self) -> &str {
            &self.user_password_file_name
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn create_config() {
            assert_eq!("./", Config::new().get_root_path());
            assert_eq!("standard", Config::new().get_secret_key());
        }
    }
}
