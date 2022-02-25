pub mod resource_add_form {
    use std::io::empty;

    use eframe::egui::{self, CtxRef, Button};

    use crate::{form::form::Form, resource_add::{resource_add_service::resource_add_service::ResourceAddService, resource_add_form_facade::resource_add_form_facade::ResourcseAddFormFacade}, REGISTRY, resource::resource::resource::Resource, settings::settings::{COLOR_WHITE, COLOR_RED, COLOR_GREEN}};

    pub struct ResourceAddForm {
        resource_add_service: ResourceAddService
    }

    impl ResourceAddForm {
        pub fn new() -> ResourceAddForm {
            ResourceAddForm {
                resource_add_service: ResourceAddService::new()
            }
        }

        fn click_btn_add(&self, ui: &mut eframe::egui::Ui) {
            
            ResourcseAddFormFacade::set_add_error_msg("".to_string());

            let new_resource_name = REGISTRY.lock().unwrap().form_data.resource_add.new_resource_name.clone();
            let new_template_name = REGISTRY.lock().unwrap().form_data.resource_add.new_template_name.clone();
            let login = REGISTRY.lock().unwrap().auth_data.login.clone();
    
    
            if (new_resource_name.is_empty() || new_template_name.is_empty()) {
                ResourcseAddFormFacade::set_add_error_msg("Нужно ввести ресурс и пароль".to_string())
            }
    
            let resource = Resource::new(
                new_resource_name, 
                new_template_name,
                login
            );
    
            match self.resource_add_service.resource_add(resource) {
                Ok(_line) => (),
                Err(e) => ResourcseAddFormFacade::set_add_error_msg(e.to_string()),
            };
        }
    }

    impl Form for ResourceAddForm {
        fn render(&self, ui: &mut eframe::egui::Ui, _ctx: &CtxRef) {
            ui.group(|ui| {
                
                ui.set_max_height(50.0);

                ui.with_layout(egui::Layout::left_to_right(), |ui| {
    
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
                    let add_btn = ui.add(Button::new("Добавить шаблон").text_color(COLOR_GREEN));

                    if !add_btn.clicked() {
                        ui.colored_label(COLOR_RED, ResourcseAddFormFacade::btn_add_error_msg());
                        return;
                    }

                    self.click_btn_add(ui)
                });
            });

        }
    }
}
