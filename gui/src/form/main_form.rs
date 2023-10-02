pub mod main_form {
    use eframe::egui::{
        TopBottomPanel, self, Context,
    };

    use crate::{
        form::form::Form, menu::form::menu_form::MenuForm, footer::form::footer_form::FooterForm,
    };

    pub struct GUI {}

    impl GUI {
        pub fn render(mut form: impl Form, ui: &mut egui::Ui, ctx: &Context) {
            form.render(ui, ctx);
        }
    }

    impl GUI {
        pub fn new() -> GUI {
            GUI {}
        }

        pub(crate) fn render_top_panel(&self, ctx: &Context) {
            // define a TopBottomPanel widget
            TopBottomPanel::top("top_panel").show(ctx, |ui| {
                ui.add_space(10.);
                GUI::render(MenuForm::new(), ui, ctx);
                ui.add_space(10.);
            });
        }

        pub(crate) fn render_bottom_panel(&self, ctx: &Context) {
            // define a TopBottomPanel widget
            TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
                // ui.add_space(10.);
                GUI::render(FooterForm::new(), ui, ctx);
                // ui.add_space(10.);
            });
        }
    }
}
