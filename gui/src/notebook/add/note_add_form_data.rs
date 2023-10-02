pub mod note_add_form_data {
 
    pub struct NoteAddFormData {
        pub id: u64,
        pub date: String,
        pub title: String,
        pub text: String,
        pub mileage: String,
        pub cost: String,
        pub tags: Vec<String>,
        pub create_tag: String,
        
        
        pub show_confirm_delete_window: bool,
        pub show_confirm_delete_note_window: bool,

        pub tag_for_delete: String,
        pub note_for_delete: u64,

        pub tag_list: Vec<String>,

        error_msg: String,
        delete_error_msg: String,
        decrypt_error_msg: String
    }

    impl NoteAddFormData {
        pub fn new() -> Self {
            Self {
                id: 0,
                date: "сегодня".to_string(),
                title: "".to_string(),
                text: "".to_string(),
                mileage: "93000".to_string(),
                cost: "0".to_string(),
                tags: vec![],
                error_msg: "".to_string(),

                create_tag: "".to_string(),

                tag_list: vec![],


                // current_resource_name: "".to_string(),
                // current_template_pass: "".to_string(),
                // current_resource_login: "".to_string(),

                show_confirm_delete_window: false,
                show_confirm_delete_note_window: false,
                note_for_delete: 0,
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

        pub fn note_for_delete(&self) -> u64 {
            self.note_for_delete
        }

        pub fn set_note_for_delete(&mut self, data: u64) {
            self.note_for_delete = data;
        }

    }
}