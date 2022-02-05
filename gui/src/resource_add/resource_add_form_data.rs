pub mod resource_add_form_data {
 
    pub struct ResourcseAddFormData {
        pub new_resource_name: String,
        pub new_template_name: String,
        pub error_msg: String,
    }

    impl ResourcseAddFormData {
        pub fn new() -> Self {
            tracing::error!("AuthData new");
            Self {
                new_resource_name: "".to_string(),
                new_template_name: "".to_string(),
                error_msg: "".to_string(),
            }
        }

        // pub fn enter_type_eq(&self, enter_type: Login) -> bool {
        //     self.enter_type == enter_type
        // }

        // pub fn set_enter_type(&mut self, enter_type: Login) {
        //     self.enter_type = enter_type
        // }

        pub fn error_msg(&self) -> String {
            self.error_msg.clone()
        }

        pub fn set_error_msg(&mut self, err: String) {
            self.error_msg = err;
        }
    }
}