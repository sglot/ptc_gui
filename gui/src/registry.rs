pub mod registry {
    use std::sync::Mutex;

    use crate::{REGISTRY, config};
    use crate::auth::form::form::LoginFormData;
    use crate::auth::auth::LastUser;
    use crate::config::config::Config;
    use crate::form::form::FormName;

    pub struct Registry {
        pub login: LoginFormData,
        pub current_form: Mutex<FormName>,
        pub last_user: LastUser,
        pub config: Mutex<Config>,
    }

    impl<'a> Registry {
        pub fn new() -> Self {
            let config = Config::new();
            // let mut key = config.get_secret_key();
            // let cryptor = Cryptor { key: &mut key };
            // let maker = MakerFS::new(config.clone());
            // let auth = AuthFs::new(cryptor.clone(), maker.clone());

            let last_user:LastUser = confy::load_path("./last-users-list.tmp").unwrap_or_default();
            tracing::error!("loaded  {:?}", last_user);
            
            Registry {
                login: LoginFormData::new(),
                current_form: Mutex::new(FormName::Auth),
                last_user,
                config: Mutex::new(config),
            }
        }

        pub fn config() -> Config {
            REGISTRY.lock().unwrap().config.lock().unwrap().clone()
        }

        pub fn eq_current_form(form_name: FormName) -> bool {
            form_name == *REGISTRY.lock().unwrap().current_form.lock().unwrap()
        }

        pub fn set_current_form(form_name: FormName) {
            *REGISTRY.lock().unwrap().current_form.lock().unwrap() = form_name;
        }
    }
}

