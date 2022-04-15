pub mod registry_repository {
    use crate::auth::auth_data::auth_data::AuthData;
    use crate::auth::{auth::LastUser};
    use crate::form::form::FormData;
    use crate::{config::config::Config};
    use crate::{form::form::FormName};
            
    pub struct RegistryRepository {
        pub auth_data: AuthData,
        pub form_data: FormData,
        pub current_form: FormName,
        pub last_user: LastUser,
        pub config: Config,
    }

    impl<'a> RegistryRepository {
        pub fn new() -> Self {
            let config = Config::new();

            let last_user:LastUser = confy::load_path("./last-users-list.tmp").unwrap_or_default();
            
            RegistryRepository {
                auth_data: AuthData::new(),
                form_data: FormData::new(),
                current_form: FormName::Auth,
                last_user,
                config: config,
            }
        }
    }
}

