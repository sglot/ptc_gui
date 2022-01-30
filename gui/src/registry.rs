pub mod registry {
    use crate::{REGISTRY};
    use crate::{config::config::Config};
    use crate::{form::form::FormName};
            
    pub struct Registry {
    }

    impl<'a> Registry {
        
        pub fn config() -> Config {
            REGISTRY.lock().unwrap().config.clone()
        }

        pub fn eq_current_form(form_name: FormName) -> bool {
            form_name == REGISTRY.lock().unwrap().current_form
        }

        pub fn set_current_form(form_name: FormName) {
            REGISTRY.lock().unwrap().current_form = form_name;
        }

        // pub fn auth_data() -> Result<&'a mut AuthData, std::sync::PoisonError<&'a AuthData>> {
        //     REGISTRY.lock().unwrap().auth_data.get_mut()
        // }
    }
}

