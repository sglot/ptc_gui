pub mod auth_service {

    use crate::{config::config::Config, cryptor::cryptor::Cryptor, registry::registry::Registry, support::fs::maker::maker_fs::MakerFS, auth::auth_fs::auth_fs::AuthFs, user::{user_repositiry_fs::user_repositiry_fs::UserRepositoryFS, user::user::User}};
    pub struct AuthService {
        cryptor: Box<Cryptor>,
        auth: AuthFs
    }

    impl AuthService {
        pub fn new() -> AuthService {
            let cryptor = Cryptor { key: Registry::config().get_secret_key() };
            let maker = MakerFS::new(Registry::config());
            let auth = AuthFs::new(cryptor.clone(), maker.clone());

            AuthService {
                cryptor: Box::new(cryptor),
                auth
            }
        }

        pub fn reg(&mut self, login: String, pass: String) -> Result<String, String> {
            let config = Config::new();

            let user = User::new(&login, &pass);
            let user_repository = UserRepositoryFS::new(&config);

            match user_repository.exists(user) {
                true => Err("Такой пользователь уже существует".to_string()),
                false => {
                    self.cryptor.set_key(pass.to_string());
                    match user_repository.save(&login, &self.cryptor.crypt(&pass.to_string())) {
                        Ok(res) => {Ok(res)},
                        Err(e) => {Err(e.to_string())},
                    }
                }
            }
        }

        pub fn authenticate(&mut self, login: String, pass: String) -> Result<String, String> {
            match self.auth.check_auth(&login, &pass) {
                Ok(res) => if res {
                    Ok("Успех".to_string())
                } else {
                    return Err("Ошибка".to_string());
                }
                Err(e) => Err(e),
            }
            
        }

        // pub fn check_auth(login: String, pass: String) -> Result<String, String> {

        // }
    }

    
}
