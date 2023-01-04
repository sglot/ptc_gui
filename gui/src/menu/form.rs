pub mod menu_form {
    use eframe::egui::{self, menu::SubMenu, Button, Context, Label, Layout, RichText};

    use crate::{
        form::form::{Form, FormName},
        registry::registry::Registry,
        settings::settings::COLOR_RED,
    };

    pub struct MenuForm {}

    impl MenuForm {
        pub fn new() -> MenuForm {
            MenuForm {
                // resource_add_service: ResourceAddService::new()
            }
        }
    }

    impl Form for MenuForm {
        fn render(&mut self, ui: &mut eframe::egui::Ui, _ctx: &Context) {
            egui::menu::bar(ui, |ui| {
                // controls

                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.with_layout(Layout::left_to_right(), |ui| {
                        egui::menu::menu_button(ui, RichText::new("Пароли"), |ui| {
                            ui.add(
                                Button::new("Изменить пароль"), // .text_style(egui::TextStyle::Body)
                            );
                            ui.add(
                                Button::new("Использовать ключ"), // .text_style(egui::TextStyle::Body)
                            );
                        });
                    });

                    ui.with_layout(Layout::right_to_left(), |ui| {
                        let close_btn = ui.add(
                            Button::new(RichText::new("❌")), // .text_style(egui::TextStyle::Body)
                        );

                        let refresh_btn = ui.add(
                            Button::new("🔄"), // .text_style(egui::TextStyle::Body)
                        );
                        
                        if refresh_btn.clicked() {
                            Registry::set_current_form(FormName::Auth);
                        }

                        if close_btn.clicked() {
                            std::process::exit(0);
                        }

                        // let _theme_btn =
                        //     ui.add(Button::new("🌙").text_style(egui::TextStyle::Body));
                    });
                });
            });
        }
    }
}
