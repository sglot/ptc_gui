pub mod user_repositiry_fs {
    use crate::config::config::Config;
    use crate::support::fs::maker::maker_fs::MakerFS;
    use crate::user::user::user::User;
    use std::fs;

    pub struct UserRepositoryFS<'a> {
        config: &'a Config,
        maker: MakerFS,
    }

    impl<'a> UserRepositoryFS<'a> {
        pub fn new(config: &'a Config) -> UserRepositoryFS {
            let maker = MakerFS::new(config.clone());
            UserRepositoryFS { maker, config }
        }

        pub fn save(&self, path_to: &str, data: &str) -> std::io::Result<String> {
            let mut path = self.maker.make_user_home_dir(path_to);

            fs::create_dir_all(&path)?;
            path.push_str(self.config.get_password_file_name());

            use std::fs::File;
            use std::io::Write;
            let mut output = File::create(&path)?;
            write!(output, "{}", data)?;
            tracing::error!("Пользователь успешно создан");
            Ok(String::from("Пользователь успешно создан"))
        }

        pub fn exists(&self, user: User) -> bool {
            use std::path::Path;
            Path::new(&self.maker.make_user_home_dir(user.get_login())).exists()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn user_not_exists() {
            let config = Config::new();
            let user_repository = UserRepositoryFS::new(&config);

            assert_eq!(false, user_repository.exists(User::new("unexists_user", "pass")));
        }

        #[test]
        fn user_exists() {
            let config = Config::new();
            let _maker = MakerFS::new(config.clone());
            let user_repository = UserRepositoryFS::new(&config);
            let _dir = user_repository.save("test", "pass");

            assert_eq!(true, user_repository.exists(User::new("test", "pass")));
        }
    }
}
