pub mod footer_form {
    use eframe::egui::{ Context, Layout,};

    use crate::{
        form::form::{Form},
        generate_password::form::generate_password_form::GeneratePasswordForm,
    };

    pub struct FooterForm {}

    impl FooterForm {
        pub fn new() -> FooterForm {
            FooterForm {
            }
        }
    }

    impl Form for FooterForm {
        fn render(&mut self, ui: &mut eframe::egui::Ui, ctx: &Context) {
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.with_layout(Layout::left_to_right(), |ui| {
                        GeneratePasswordForm::new().render(ui, ctx)
                    });
                });
        }
    }
}
