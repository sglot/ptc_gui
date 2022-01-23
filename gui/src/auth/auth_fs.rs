pub mod auth_fs {

    use std::fs;

    use crate::cryptor::cryptor::Cryptor;
    use crate::support::fs::maker::maker_fs::MakerFS;

    pub struct AuthFs {
        cryptor: Box<Cryptor>,
        maker: Box<MakerFS>,
    }

    impl AuthFs {
        pub fn new(cryptor: Cryptor, maker: MakerFS) -> AuthFs {
            AuthFs {
                cryptor: Box::new(cryptor),
                maker: Box::new(maker),
            }
        }

        pub fn check_auth(&self, login: &str, pass: &str) -> Result<bool, String> {
            let pass: String = self.cryptor.crypt(&pass);

            let user_path = self.maker.make_user_path(login);
            let secret: String;
            match fs::read_to_string(user_path) {
                Ok(res) => secret = res,
                Err(_e) => return Err("Пользователь не существует".to_string()),
            }

            Ok(secret == pass)
        }
    }


    #[cfg(test)]
    mod tests {
        use crate::{config::config::Config, user::user_repositiry_fs::user_repositiry_fs::UserRepositoryFS };

        use super::*;

        #[test]
        fn auth_err() {
            let cryptor = Cryptor { key:"test".to_string() };
            let maker = MakerFS::new(Config::new());
            let auth = AuthFs::new(cryptor.clone(), maker.clone());

            assert!(auth.check_auth("1", "2").is_err());
        }

        #[test]
        fn auth_ok() {
            let config = Config::new();
            let cryptor = Cryptor { key: config.get_secret_key() };
            let maker = MakerFS::new(Config::new());
            let auth = AuthFs::new(cryptor.clone(), maker.clone());


            let user_repository = UserRepositoryFS::new(&config);
            let _dir = user_repository.save("test", &cryptor.crypt("pass"));

            assert_eq!(true, auth.check_auth("test", "pass").unwrap());
        }

        #[test]
        fn auth_false_no_crypt() {
            let config = Config::new();
            let cryptor = Cryptor { key: config.get_secret_key() };
            let maker = MakerFS::new(Config::new());
            let auth = AuthFs::new(cryptor.clone(), maker.clone());


            let user_repository = UserRepositoryFS::new(&config);
            let _dir = user_repository.save("test", "pass");

            assert_eq!(false, auth.check_auth("test", "pass").unwrap());
        }
    }
}
