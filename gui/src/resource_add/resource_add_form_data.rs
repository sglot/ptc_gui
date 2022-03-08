pub mod resource_add_form_data {
 
    pub struct ResourcseAddFormData {
        pub new_resource_name: String,
        pub new_template_pass: String,
        pub new_resource_login: String,
        error_msg: String,
        
        pub show_confirm_delete_window: bool,
        pub is_resource_field_changed: bool,
        pub is_show_pass: bool,
        delete_error_msg: String,
        decrypt_error_msg: String,

        pub current_resource_name: String,
        pub current_template_pass: String,
        pub current_resource_login: String,
    }

    impl ResourcseAddFormData {
        pub fn new() -> Self {
            Self {
                new_resource_name: "".to_string(),
                new_template_pass: "".to_string(),
                new_resource_login: "".to_string(),
                error_msg: "".to_string(),

                current_resource_name: "".to_string(),
                current_template_pass: "".to_string(),
                current_resource_login: "".to_string(),

                show_confirm_delete_window: false,
                is_resource_field_changed: false,
                is_show_pass: false,
                delete_error_msg: "".to_string(),
                decrypt_error_msg: "".to_string(),
            }
        }

        pub fn error_msg(&self) -> String {
            self.error_msg.clone()
        }

        pub fn set_error_msg(&mut self, err: String) {
            self.error_msg = err;
        }

        pub fn delete_error_msg(&self) -> String {
            self.delete_error_msg.clone()
        }

        pub fn set_delete_error_msg(&mut self, err: String) {
            self.delete_error_msg = err;
        }

        pub fn decrypt_error_msg(&self) -> String {
            self.decrypt_error_msg.clone()
        }

        pub fn set_decrypt_error_msg(&mut self, err: String) {
            self.decrypt_error_msg = err;
        }
    }
}