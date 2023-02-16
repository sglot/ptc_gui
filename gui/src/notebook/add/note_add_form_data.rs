pub mod note_add_form_data {
 
    pub struct NoteAddFormData {
        pub new_date: String,
        pub new_title: String,
        pub new_text: String,
        pub new_mileage: String,
        pub new_cost: String,
        pub new_tags: Vec<String>,

        pub create_tag: String,
        error_msg: String,
        
        pub show_confirm_delete_window: bool,
        pub tag_for_delete: String,
        // pub is_resource_field_changed: bool,
        // pub is_show_pass: bool,
        delete_error_msg: String,
        decrypt_error_msg: String,
        pub tag_list: Vec<String>,

        // pub current_resource_name: String,
        // pub current_template_pass: String,
        // pub current_resource_login: String,
    }

    impl NoteAddFormData {
        pub fn new() -> Self {
            Self {
                new_date: "сегодня".to_string(),
                new_title: "".to_string(),
                new_text: "".to_string(),
                new_mileage: "93000".to_string(),
                new_cost: "0".to_string(),
                new_tags: vec![],
                error_msg: "".to_string(),

                create_tag: "".to_string(),

                tag_list: vec![],


                // current_resource_name: "".to_string(),
                // current_template_pass: "".to_string(),
                // current_resource_login: "".to_string(),

                show_confirm_delete_window: false,
                tag_for_delete: "".to_string(),
                // is_resource_field_changed: false,
                // is_show_pass: false,
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

        pub fn tag_for_delete(&self) -> String {
            self.tag_for_delete.clone()
        }

        pub fn set_tag_for_delete(&mut self, data: String) {
            self.tag_for_delete = data;
        }

    }
}