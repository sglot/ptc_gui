pub mod auth_fs {

    use std::fs;
    use std::io::{self};

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

        pub fn check_auth(&self, login: &str) -> bool {
            println!("Введите пароль");
            let mut pass = String::new();
            io::stdin().read_line(&mut pass).unwrap();
            pass = pass.trim_end().to_string();
            pass = self.cryptor.crypt(&pass);

            let user_path = self.maker.make_user_path(login);
            let secret = fs::read_to_string(user_path).expect("Ошибка чтения пароля");

            secret == pass
        }
    }
}
