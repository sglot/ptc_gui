pub mod main_form;

pub mod form {
    use eframe::egui::{ Context};

    use crate::{resource_add::resource_add_form_data::resource_add_form_data::ResourcseAddFormData, resource_list::resource_list_form_data::resource_list_form_data::ResourcseListFormData, generate_password::generate_password_form_data::generate_password_form_data::GeneratePasswordFormData};

    #[derive(PartialEq)]
    pub enum FormName {
        Auth,
        ResourceList,
    }

    pub struct FormData {
        pub resource_add: ResourcseAddFormData,
        pub resource_list: ResourcseListFormData,
        pub generate_password: GeneratePasswordFormData,
    }

    impl FormData {
        pub fn new() -> Self {
            Self {
                resource_add: ResourcseAddFormData::new(),
                resource_list: ResourcseListFormData::new(),
                generate_password: GeneratePasswordFormData::new(),
            }
        }
    }

    pub trait Form {
        fn render(&mut self, ui: &mut eframe::egui::Ui, ctx: &Context);
    }
}
