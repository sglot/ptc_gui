pub mod main_form;

pub mod form {
    use crate::resource_add::resource_add_form_data::resource_add_form_data::ResourcseAddFormData;

    #[derive(PartialEq)]
    pub enum FormName {
        Auth,
        ResourceList,
    }

    pub struct FormData {
        pub resource_add: ResourcseAddFormData,
    }

    impl FormData {
        pub fn new() -> Self {
            Self {
                resource_add: ResourcseAddFormData::new(),
            }
        }
    }

    pub trait Form {
        fn render(&self, ui: &mut eframe::egui::Ui);
    }
}
