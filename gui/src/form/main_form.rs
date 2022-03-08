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
        registry::registry::Registry, settings::settings::COLOR_RED, menu::form::menu_form::MenuForm,
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
                GUI::render(MenuForm::new(), ui, ctx);
                ui.add_space(10.);
            });
        }
    }
}
