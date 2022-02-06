pub mod resource_add_form {
    use eframe::egui::{Color32, Separator, self, Direction};

    use crate::{form::form::Form, resource_add::resource_add_service::resource_add_service::ResourceAddService, REGISTRY, resource::resource::resource::Resource};

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
                
                ui.set_max_height(50.0);
                // ui.set_max_width(20.0);


                ui.with_layout(egui::Layout::left_to_right(), |ui| {
                    // ui.set_max_width(200.);
    
                    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                        ui.set_max_width(200.);
                        ui.label("Ресурс:");
                        ui.text_edit_singleline(&mut REGISTRY.lock().unwrap().form_data.resource_add.new_resource_name);
                    });

                    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                        ui.set_max_width(200.);
                        ui.label("Шаблон:");
                        ui.text_edit_singleline(&mut REGISTRY.lock().unwrap().form_data.resource_add.new_template_name);
                    });
                    
                    tracing::error!("Добавить шаблон");
                    if !ui.button("Добавить шаблон").clicked() {
                        ui.label(REGISTRY.lock().unwrap().form_data.resource_add.error_msg());
                        return;
                    }
                    tracing::error!("WHITE");
                    const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
                
                    let new_resource_name = REGISTRY.lock().unwrap().form_data.resource_add.new_resource_name.clone();
                    let new_template_name = REGISTRY.lock().unwrap().form_data.resource_add.new_template_name.clone();
                    let login = REGISTRY.lock().unwrap().auth_data.login.clone();


                    let resource = Resource::new(
                        new_resource_name, 
                        new_template_name,
                        login
                    );

                    match self.resource_add_service.resource_add(resource) {
                        Ok(line) => ui.colored_label(WHITE, line),
                        Err(e) => ui.colored_label(WHITE, "Ошибка ввода".to_string()),
                    };
                });
            });

            

        
        }
    }
}
