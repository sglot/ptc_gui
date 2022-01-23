pub mod resource_list_form {
    use eframe::egui::{Color32, Separator};

    use crate::{form::form::Form};

    pub struct ResourceListForm {}

    impl ResourceListForm {
        pub fn new() -> ResourceListForm {
            ResourceListForm {}
        }
    }

    impl Form for ResourceListForm {
        fn render(&self, ui: &mut eframe::egui::Ui) {
            ui.set_row_height(10.0);
            ui.set_width(300.0);
            const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
            const PADDING: f32 = 10.;

            let iter = (0..2).map(|a| -> String {
                "s".to_string()
            });

            (iter).into_iter().for_each(|a| {
                ui.add_space(0.5);
                // render title
                let login = format!("▶ ddddddddddd {}", a);
                ui.colored_label(WHITE, login);

                ui.add_space(PADDING);
                ui.add(Separator::default());
            });
        
        }
    }
}
