pub mod resource_list_form {
    use eframe::egui::{Color32, Separator, self};

    use crate::{form::form::Form, resource_list::resource_list_service::resource_list_service::ResourceListService, REGISTRY};

    pub struct ResourceListForm {
        resource_list_service: ResourceListService
    }

    impl ResourceListForm {
        pub fn new() -> ResourceListForm {
            ResourceListForm {
                resource_list_service: ResourceListService::new()
            }
        }
    }

    impl Form for ResourceListForm {
        fn render(&self, ui: &mut eframe::egui::Ui) {

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(30.);
                let login = REGISTRY.lock().unwrap().auth_data.login.clone();
            
                match self.resource_list_service.resource_list(&login) {
                    Ok(line) => ui.colored_label(WHITE, line),
                    Err(e) => ui.colored_label(WHITE, "Пусто".to_string()),
                };
            });
            
            ui.add_space(10.);
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
