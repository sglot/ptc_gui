pub mod main_form {
    use eframe::egui::{
        self,
        Button,
        CtxRef,
        Label,
        Layout,
        TopBottomPanel,
    };

    use crate::{
        form::form::{Form, FormName},
        registry::registry::Registry,
    };

    pub struct GUI {}

    impl GUI {
        pub fn render(form: impl Form, ui: &mut eframe::egui::Ui, ctx: &CtxRef) {
            form.render(ui, ctx);
        }
    }

    impl GUI {
        pub fn new() -> GUI {
            GUI {}
        }

        pub(crate) fn render_top_panel(&self, ctx: &CtxRef) {
            // define a TopBottomPanel widget
            TopBottomPanel::top("top_panel").show(ctx, |ui| {
                ui.add_space(10.);
                egui::menu::bar(ui, |ui| {
                    // logo
                    ui.with_layout(Layout::left_to_right(), |ui| {
                        ui.add(Label::new("📓").text_style(egui::TextStyle::Heading));
                    });
                    // controls
                    ui.with_layout(Layout::right_to_left(), |ui| {
                        let _close_btn =
                            ui.add(Button::new("❌").text_style(egui::TextStyle::Body));

                        let refresh_btn =
                            ui.add(Button::new("🔄").text_style(egui::TextStyle::Body));

                        if refresh_btn.clicked() {
                            Registry::set_current_form(FormName::Auth);
                        }

                        let _theme_btn =
                            ui.add(Button::new("🌙").text_style(egui::TextStyle::Body));
                    });
                });
                ui.add_space(10.);
            });
        }
    }
}
