pub mod resource_add_form {
    use eframe::egui::{Color32, Separator, self, Direction};

    use crate::{form::form::Form, resource_add::resource_add_service::resource_add_service::ResourceAddService, REGISTRY};

    pub struct ResourceAddForm {
        resource_add_service: ResourceAddService
    }

    impl ResourceAddForm {
        pub fn new() -> ResourceAddForm {
            ResourceAddForm {
                resource_add_service: ResourceAddService::new()
            }
        }
    }

    impl Form for ResourceAddForm {
        fn render(&self, ui: &mut eframe::egui::Ui) {
            ui.group(|ui| {
                
                // ui.set_min_height(100.0);
                // ui.set_max_width(20.0);


                ui.with_layout(egui::Layout::centered_and_justified(Direction::TopDown), |ui| {
                    ui.add_space(3.);
                    ui.set_max_width(200.);
                    let login = REGISTRY.lock().unwrap().auth_data.login.clone();
    
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                        
                        ui.label("Ресурс:");
                        ui.text_edit_singleline(&mut REGISTRY.lock().unwrap().form_data.resource_add.new_resource_name);
                    });

                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                        ui.label("Шаблон:");
                        ui.text_edit_singleline(&mut REGISTRY.lock().unwrap().form_data.resource_add.new_template_name);
                    });
                    
                    
                    if !ui.button("Добавить шаблон").clicked() {
                        ui.label(REGISTRY.lock().unwrap().form_data.resource_add.error_msg());
                        return;
                    }
                    const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
                
                    match self.resource_add_service.resource_add(&login) {
                        Ok(line) => ui.colored_label(WHITE, line),
                        Err(e) => ui.colored_label(WHITE, "Ошибка ввода".to_string()),
                    };
                });
            });

            

        
        }
    }
}
