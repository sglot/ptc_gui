pub mod menu_form {
    use eframe::egui::{self, CtxRef, Label, Layout, Button};

    use crate::{form::form::{Form, FormName}, settings::settings::COLOR_RED, registry::registry::Registry};

    pub struct MenuForm {
    }

    impl MenuForm {
        pub fn new() -> MenuForm {
           MenuForm {
                // resource_add_service: ResourceAddService::new()
            }
        }
    }

    impl Form for MenuForm {
        fn render(&self, ui: &mut eframe::egui::Ui, _ctx: &CtxRef) {
            egui::menu::bar(ui, |ui| {

                // controls
                ui.with_layout(Layout::right_to_left(), |ui| {
                    let _close_btn =
                        ui.add(Button::new("❌").text_style(egui::TextStyle::Body));

                    let refresh_btn =
                        ui.add(Button::new("🔄").text_style(egui::TextStyle::Body));

                    if refresh_btn.clicked() {
                        Registry::set_current_form(FormName::Auth);
                    }

                    // let _theme_btn =
                    //     ui.add(Button::new("🌙").text_style(egui::TextStyle::Body));
                });
            });

        }
    }
}
