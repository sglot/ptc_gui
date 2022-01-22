pub mod auth_service {

    use crate::{config::config::Config, user::user::User, user_repositiry_fs::user_repositiry_fs::UserRepositoryFS, cryptor::cryptor::Cryptor, registry::registry::Registry, support::fs::maker::maker_fs::MakerFS, auth::auth_fs::auth_fs::AuthFs};
    pub struct AuthService {
        cryptor: Box<Cryptor>,
        auth: AuthFs
    }

    impl AuthService {
        pub fn new() -> AuthService {
            
            // let mut key = Registry::config().lock().unwrap().get_secret_key().clone();
            let cryptor = Cryptor { key: Registry::config().get_secret_key() };
            let maker = MakerFS::new(Registry::config());
            let auth = AuthFs::new(cryptor.clone(), maker.clone());

            AuthService {
                cryptor: Box::new(cryptor),
                auth
            }
        }

        pub fn reg(&self, login: String, pass: String) -> Result<String, String> {
            let config = Config::new();

            let user = User::new(&login, &pass);
            let user_repository = UserRepositoryFS::new(&config);
            tracing::error!("1");
            match user_repository.exists(user) {
                true => Err("Такой пользователь уже существует".to_string()),
                false => {
                    // let mut key = config.get_secret_key();
                    // let cryptor = Cryptor { key: key };
                    tracing::error!("2");
                    match user_repository.save(&login, &self.cryptor.crypt(&pass.to_string())) {
                        Ok(res) => {tracing::error!("3");Ok(res)},
                        Err(e) => {tracing::error!("4");Err(e.to_string())},
                    }
                }
            }
        }

        // pub fn check_auth(login: String, pass: String) -> Result<String, String> {

        // }
    }
}
