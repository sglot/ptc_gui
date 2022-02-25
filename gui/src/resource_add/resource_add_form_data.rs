pub mod resource_add_form_data {
 
    pub struct ResourcseAddFormData {
        pub new_resource_name: String,
        pub new_template_name: String,
        error_msg: String,
        
        pub show_confirm_delete_window: bool,
        delete_error_msg: String,
        decrypt_error_msg: String,

        pub current_resource_name: String,
        pub current_template_name: String,
    }

    impl ResourcseAddFormData {
        pub fn new() -> Self {
            Self {
                new_resource_name: "".to_string(),
                new_template_name: "".to_string(),
                error_msg: "".to_string(),

                current_resource_name: "".to_string(),
                current_template_name: "".to_string(),

                show_confirm_delete_window: false,
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