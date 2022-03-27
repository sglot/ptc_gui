pub mod resource_add_form {
    use eframe::{egui::{self, Context, Button, WidgetText, RichText},};

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
            let new_template_pass = REGISTRY.lock().unwrap().form_data.resource_add.new_template_pass.clone();
            let new_resource_login = REGISTRY.lock().unwrap().form_data.resource_add.new_resource_login.clone();
            let login = REGISTRY.lock().unwrap().auth_data.login.clone();
    
    
            if (new_resource_name.is_empty() || new_template_pass.is_empty() || new_resource_login.is_empty()) {
                ResourcseAddFormFacade::set_add_error_msg("Нужно ввести ресурс, пароль, логин".to_string())
            }
    
            let resource = Resource::new(
                new_template_pass, 
                new_resource_login, 
                // "master_p".to_string(), 
                new_resource_name, 
                login
            );
    
            match self.resource_add_service.resource_add(resource) {
                Ok(_line) => (),
                Err(e) => ResourcseAddFormFacade::set_add_error_msg(e.to_string()),
            };
        }
    }

    impl Form for ResourceAddForm {
        fn render(&self, ui: &mut eframe::egui::Ui, _ctx: &Context) {
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
                        ui.label("Логин:");
                        ui.text_edit_singleline(&mut REGISTRY.lock().unwrap().form_data.resource_add.new_resource_login);
                    });

                    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                        ui.set_max_width(200.);
                        ui.label("Пароль:");
                        ui.text_edit_singleline(&mut REGISTRY.lock().unwrap().form_data.resource_add.new_template_pass);
                    });

                    tracing::error!("Добавить пароль");
                    let add_btn = ui.add(Button::new(RichText::new("Добавить пароль").color(COLOR_GREEN) ));

                    if !add_btn.clicked() {
                        
                        return;
                    }

                    self.click_btn_add(ui)
                });

                // TODO: переделать в ошибки для каждого поля
                ui.colored_label(COLOR_RED, ResourcseAddFormFacade::btn_add_error_msg());
            });

        }
    }
}
