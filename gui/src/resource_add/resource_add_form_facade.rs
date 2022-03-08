pub mod resource_add_form_facade {
    use crate::REGISTRY;

    pub struct ResourcseAddFormFacade {}

    impl ResourcseAddFormFacade {
        pub fn btn_add_error_msg() -> String {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .error_msg()
                .clone()
        }

        pub fn set_add_error_msg(err: String) {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .set_error_msg(err);
        }

        pub fn delete_error_msg() -> String {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .delete_error_msg()
                .clone()
        }

        pub fn set_delete_error_msg(err: String) {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .set_delete_error_msg(err);
        }

        pub fn decrypt_error_msg() -> String {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .decrypt_error_msg()
        }

        pub fn set_decrypt_error_msg(err: String) {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .set_decrypt_error_msg(err);
        }

        pub fn current_resource_name() -> String {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .current_resource_name
                .clone()
        }

        pub fn set_current_resource_name(data: String) {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .current_resource_name = data;
        }

        pub fn current_template_pass() -> String {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .current_template_pass
                .clone()
        }

        pub fn set_current_template_pass(data: String) {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .current_template_pass = data;
        }

        pub fn new_template_pass() -> String {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .new_template_pass
                .clone()
        }

        pub fn set_new_template_pass(data: String) {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .new_template_pass = data;
        }

        pub fn current_resource_login() -> String {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .current_resource_login
                .clone()
        }

        pub fn set_current_resource_login(data: String) {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .current_resource_login = data;
        }

        pub fn new_resource_login() -> String {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .new_resource_login
                .clone()
        }

        pub fn set_new_resource_login(data: String) {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .new_resource_login = data;
        }

        pub fn new_resource_name() -> String {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .new_resource_name
                .clone()
        }

        pub fn set_new_resource_name(data: String) {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .new_resource_name = data;
        }

        pub fn show_confirm_delete_window() -> bool {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .show_confirm_delete_window
                .clone()
        }

        pub fn set_show_confirm_delete_window(data: bool) {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .show_confirm_delete_window = data;
        }

        pub fn is_resource_field_changed() -> bool {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .is_resource_field_changed
                .clone()
        }

        pub fn set_resource_field_changed(data: bool) {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .is_resource_field_changed = data;
        }

        pub fn is_show_pass() -> bool {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .is_show_pass
                .clone()
        }

        pub fn set_default_show_pass() {
            REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .is_show_pass = false;
        }
    }
}
