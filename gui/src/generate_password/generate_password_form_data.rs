pub mod generate_password_form_data {
 
    pub struct GeneratePasswordFormData {
        pub chars_count: String,
        pub generated_pass: String,
    }

    impl GeneratePasswordFormData {
        pub fn new() -> Self {
            Self {
                chars_count: "12".to_string(),
                generated_pass: "".to_string(),
            }
        }
    }
}