pub mod menu_form {
    use eframe::egui::{self, Button, Context, Layout, RichText};

    use crate::{
        form::form::{Form, FormName},
        registry::registry::Registry, settings::settings::{COLOR_RED, COLOR_YELLOW},
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
            if !Registry::is_auth() {
                return;
            }

            egui::menu::bar(ui, |ui| {
                // controls

                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.with_layout(Layout::left_to_right(), |ui| {
                        let passwords_btn = ui.add(
                            Button::new("Пароли"),
                        );
                        
                        if passwords_btn.clicked() && Registry::is_auth() {
                            Registry::set_current_form(FormName::ResourceList);
                        }

                        let notebook_btn = ui.add(
                            Button::new("Записи"),
                        );
                        
                        if notebook_btn.clicked() && Registry::is_auth() {
                            Registry::set_current_form(FormName::Notebook);
                        }
                    });

                    ui.with_layout(Layout::right_to_left(), |ui| {
                        let close_btn = ui.add(
                            Button::new(RichText::new("❌").color(COLOR_YELLOW)),
                        );

                        let refresh_btn = ui.add(
                            Button::new("🔄"),
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
