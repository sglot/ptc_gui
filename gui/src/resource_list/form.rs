pub mod resource_list_form {
    use eframe::egui::{self, Color32, ScrollArea, Separator};

    use crate::{
        form::{form::Form, main_form::main_form::GUI},
        resource_add::form::resource_add_form::ResourceAddForm,
        resource_list::resource_list_service::resource_list_service::ResourceListService,
        REGISTRY,
    };
    const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
    const PADDING: f32 = 10.;
    pub struct ResourceListForm {
        resource_list_service: ResourceListService,
    }

    impl ResourceListForm {
        pub fn new() -> ResourceListForm {
            ResourceListForm {
                resource_list_service: ResourceListService::new(),
            }
        }

        fn render_resource_list(&self, ui: &mut eframe::egui::Ui) {
            const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
            ui.group(|ui| {
                ui.set_max_width(200.0);
                ui.set_min_width(100.0);
            ScrollArea::vertical().show(ui, |ui: &mut eframe::egui::Ui| {
                
                

                ui.vertical(|ui: &mut eframe::egui::Ui| {
                    
                        ui.label("Ресурсы:");

                        ui.add_space(10.);
                        let login = REGISTRY.lock().unwrap().auth_data.login.clone();

                        match self.resource_list_service.resource_list(&login) {
                            Ok(recources) => self.list(recources, ui),
                            Err(e) => {
                                ui.colored_label(WHITE, "Пусто".to_string());
                                ()
                            }
                        };
                    });
                });
            });
        }

        fn list(&self, list: Vec<String>, ui: &mut eframe::egui::Ui) {
            ui.add_space(10.);

            for resource in list {

                if ui.selectable_value(
                    &mut REGISTRY.lock().unwrap().form_data.resource_add.new_resource_name, 
                    resource.clone(),
                    &resource
                ).clicked() {
                    let login = REGISTRY.lock().unwrap().auth_data.login.clone();

                    REGISTRY.lock().unwrap().form_data.resource_add.new_resource_name = resource.clone();
                    REGISTRY.lock().unwrap().form_data.resource_add.new_template_name = match self.resource_list_service
                    .find_template(&login, &resource) {
                        Ok(res) => res,
                        Err(_) => todo!(),
                    }
                }

                ui.add_space(PADDING);
                ui.add(Separator::default());
                ui.add_space(PADDING);
            }
        }

        fn edit_area(&self, ui: &mut eframe::egui::Ui) {
            ui.add_space(10.);

            ui.text_edit_multiline(
                &mut REGISTRY
                    .lock()
                    .unwrap()
                    .form_data
                    .resource_add
                    .new_template_name,
            );

            if !ui.button("Расшифровать").clicked() {
                return;
            }

            let tmpl = REGISTRY.lock().unwrap().form_data.resource_add.new_template_name.clone();
            
            REGISTRY.lock().unwrap().form_data.resource_add.new_template_name = match self.resource_list_service.decrypt_template(&tmpl) {
                Ok(template) => template,
                Err(_) => todo!(),
            }
        }
    }

    impl Form for ResourceListForm {
        fn render(&self, ui: &mut eframe::egui::Ui) {
            ui.vertical(|ui: &mut eframe::egui::Ui| {
                GUI::render(ResourceAddForm::new(), ui);

                ui.add_space(20.);

                ui.horizontal(|ui: &mut eframe::egui::Ui| {
                    ui.set_min_height(300.0);

                    self.render_resource_list(ui);
                    self.edit_area(ui);
                });
            });
        }
    }
}
