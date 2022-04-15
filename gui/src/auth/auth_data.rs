pub mod auth_data {
    use std::sync::Mutex;

    use crate::auth::auth::Login;

    pub struct AuthData {
        pub enter_type: Login,
        pub login: String,
        pub pass: Mutex<String>,
        pub error_msg: Mutex<String>,
    }

    impl AuthData {
        pub fn new() -> Self {
            Self {
                enter_type: Login::Login,
                login: "t".to_string(),
                pass: Mutex::new("t".to_string()),
                error_msg: Mutex::new("".to_string()),
            }
        }

        pub fn enter_type_eq(&self, enter_type: Login) -> bool {
            self.enter_type == enter_type
        }

        pub fn set_enter_type(&mut self, enter_type: Login) {
            self.enter_type = enter_type
        }

        pub fn error_msg(&self) -> String {
            self.error_msg.lock().unwrap().clone()
        }

        pub fn set_error_msg(&self, err: String) {
            *self.error_msg.lock().unwrap() = err;
        }
    }
}