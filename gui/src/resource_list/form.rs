pub mod resource_list_form {
    use std::sync::Arc;

    use eframe::egui::{
        self, Align, Button, Color32, CtxRef, Direction, Layout, ScrollArea, SelectableLabel,
        Separator, TextStyle, Window, Pos2, Align2, Vec2, Label,
    };

    use crate::{
        form::{form::Form, main_form::main_form::GUI},
        resource::{
            resource::resource::Resource,
            resource_repository::resource_repository::ResourceRepository,
        },
        resource_add::{
            form::resource_add_form::ResourceAddForm,
            resource_add_form_facade::resource_add_form_facade::ResourcseAddFormFacade,
        },
        resource_list::resource_list_service::resource_list_service::ResourceListService,
        settings::settings::{COLOR_RED, COLOR_WHITE},
        REGISTRY,
    };
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

        fn resource_list(&self, ui: &mut eframe::egui::Ui, ctx: &CtxRef) {
            ui.group(|ui| {
                ui.set_max_width(200.0);
                ui.set_min_width(100.0);
                ScrollArea::vertical().show(ui, |ui: &mut eframe::egui::Ui| {
                    ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                        // ui.vertical(|ui: &mut eframe::egui::Ui| {
                        // ui.set_height(20.);
                        ui.label("Ресурсы:");

                        ui.add_space(10.);
                        let login = REGISTRY.lock().unwrap().auth_data.login.clone();

                        match self.resource_list_service.resource_list(&login) {
                            Ok(recources) => self.list(recources, ui),
                            Err(e) => {
                                ui.colored_label(COLOR_WHITE, "Пусто".to_string());
                                ()
                            }
                        };
                        // });
                    });
                });
            });
        }

        fn list(&self, list: Vec<String>, ui: &mut eframe::egui::Ui) {
            for resource in list {
                if ui
                    .selectable_value(
                        &mut REGISTRY
                            .lock()
                            .unwrap()
                            .form_data
                            .resource_add
                            .current_resource_name,
                        resource.clone(),
                        &resource,
                    )
                    .clicked()
                {
                    let login = REGISTRY.lock().unwrap().auth_data.login.clone();

                    ResourcseAddFormFacade::set_current_resource_name(resource.clone());
                    ResourcseAddFormFacade::set_current_template_name(
                        match self.resource_list_service.find_template(&login, &resource) {
                            Ok(res) => res,
                            Err(e) => e.to_string(),
                        },
                    );
                }

                ui.add_space(PADDING);
                ui.add(Separator::default());
                ui.add_space(PADDING);
            }
        }

        fn edit_area(&self, ui: &mut eframe::egui::Ui) {
            ui.add_space(10.);

            ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                ui.text_edit_multiline(
                    &mut REGISTRY
                        .lock()
                        .unwrap()
                        .form_data
                        .resource_add
                        .current_template_name,
                );

                ui.colored_label(COLOR_RED, ResourcseAddFormFacade::decrypt_error_msg());

                if !ui.button("Расшифровать").clicked() {
                    return;
                }

                let tmpl = ResourcseAddFormFacade::current_template_name();

                match self.resource_list_service.decrypt_template(&tmpl) {
                    Ok(template) => {
                        ResourcseAddFormFacade::set_current_template_name(template);
                        ResourcseAddFormFacade::set_decrypt_error_msg("".to_string());
                    }
                    Err(_) => {
                        ResourcseAddFormFacade::set_decrypt_error_msg(
                            "Невозможно расшифровать".to_string(),
                        );
                    }
                };
            });
        }

        fn delete_btn(&self, ui: &mut eframe::egui::Ui, ctx: &CtxRef, resource_name: String) {
            let remove_btn = ui.add(Button::new("Удалить").text_color(COLOR_RED));

            if remove_btn.clicked() {
                ResourcseAddFormFacade::set_show_confirm_delete_window(true);
                return;
            }

            if !ResourcseAddFormFacade::show_confirm_delete_window() {
                return;
            }

            Window::new("Удаление")
            .anchor(Align2::CENTER_CENTER, Vec2::new(0., 0.))
            .show(ctx, |window_ui| {
                window_ui.set_height(50.);
                window_ui.set_row_height(100.);
                
                
                window_ui.with_layout(Layout::top_down_justified(Align::Center), |layout_ui| {
                    layout_ui.add_space(10.);
                    layout_ui.spacing_mut().button_padding = Vec2::new(10., 5.);
                    layout_ui.spacing_mut().window_padding = Vec2::new(10., 5.);
                    layout_ui.spacing_mut().item_spacing = Vec2::new(10., 10.);
                    layout_ui.add(Label::new(format!("Точно удалить {}?", resource_name)));
                    let yes_btn = layout_ui.add(Button::new("Да").text_color(COLOR_RED));

                    if yes_btn.clicked() {
                        let new_resource_name = ResourcseAddFormFacade::new_resource_name();
                        let new_template_name = ResourcseAddFormFacade::new_template_name();
                        let login = REGISTRY.lock().unwrap().auth_data.login.clone();

                        let resource = Resource::new(new_resource_name, new_template_name, login);

                        match self.resource_list_service.resource_delete(resource) {
                            Ok(_) => {
                                ResourcseAddFormFacade::set_show_confirm_delete_window(false);
                            }
                            Err(e) => {
                                ResourcseAddFormFacade::set_delete_error_msg(e.to_string());
                            }
                        };
                    }

                    let close_btn = layout_ui.add(Button::new("Нет").text_color(COLOR_WHITE));

                    if close_btn.clicked() {
                        ResourcseAddFormFacade::set_show_confirm_delete_window(false);
                        return;
                    }
                });

                window_ui.label(ResourcseAddFormFacade::delete_error_msg());
            });

        }

        fn edit_btn(&self, ui: &mut eframe::egui::Ui, _ctx: &CtxRef) {
            if ui.button("Изменить").clicked() {
                ResourcseAddFormFacade::set_show_confirm_delete_window(true);
                return;
            }
        }
    }

    impl Form for ResourceListForm {
        fn render(&self, ui: &mut eframe::egui::Ui, ctx: &CtxRef) {
            ui.vertical(|ui: &mut eframe::egui::Ui| {
                GUI::render(ResourceAddForm::new(), ui, ctx);

                ui.add_space(20.);

                ui.horizontal(|ui: &mut eframe::egui::Ui| {
                    ui.set_min_height(300.0);

                    self.resource_list(ui, ctx);
                    self.edit_area(ui);
                });

                if ResourcseAddFormFacade::current_resource_name().is_empty() {
                    return;
                }

                ui.horizontal(|ui: &mut eframe::egui::Ui| {

                    self.edit_btn(ui, ctx);
                    self.delete_btn(ui, ctx, ResourcseAddFormFacade::current_resource_name());
                });
            });
        }
    }
}
