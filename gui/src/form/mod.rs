pub mod main_form;

pub mod form {
    #[derive(PartialEq)]
    pub enum FormName {
        Auth,
        ResourceList,
    }

    pub trait Form {
        fn render(&self, ui: &mut eframe::egui::Ui);
    }
}