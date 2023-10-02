pub mod generate_password_form {
    use eframe::egui::{Button, Context, Label, Layout, RichText, TextEdit};

    use crate::{
        form::form::Form,
        generate_password::password_generator::password_generator::PasswordGenerator,
        REGISTRY,
    };

    pub struct GeneratePasswordForm {}

    impl GeneratePasswordForm {
        pub fn new() -> GeneratePasswordForm {
            GeneratePasswordForm {}
        }
    }

    impl Form for GeneratePasswordForm {
        fn render(&mut self, ui: &mut eframe::egui::Ui, _ctx: &Context) {
            ui.with_layout(Layout::left_to_right(), |ui| {
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.add(Label::new(RichText::new("Количество символов:")));
                    ui.add(TextEdit::singleline(
                        &mut REGISTRY
                            .lock()
                            .unwrap()
                            .form_data
                            .generate_password
                            .chars_count,
                    ));

                    let make_pass_btn = ui.add(Button::new(RichText::new("Сгенерировать пароль")));

                    if make_pass_btn.clicked() {
                        let count = REGISTRY
                        .lock()
                        .unwrap()
                        .form_data
                        .generate_password
                        .chars_count
                        .clone();

                        REGISTRY
                            .lock()
                            .unwrap()
                            .form_data
                            .generate_password
                            .generated_pass = PasswordGenerator::new(count).generate().clone();
                    }

                    ui.add(TextEdit::singleline(
                        &mut REGISTRY
                            .lock()
                            .unwrap()
                            .form_data
                            .generate_password
                            .generated_pass,
                    ));
                });
            });
        }
    }
}
