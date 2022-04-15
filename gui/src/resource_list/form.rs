pub mod resource_list_form {

    use eframe::egui::{
        self, Align, Align2, Button, Label, Layout, ScrollArea, Separator, Vec2, Window, Context, RichText, 
    };

    use crate::{
        form::{form::Form, main_form::main_form::GUI},
        resource::resource::resource::Resource,
        resource_add::{
            form::resource_add_form::ResourceAddForm,
            resource_add_form_facade::resource_add_form_facade::ResourcseAddFormFacade,
            resource_add_service::resource_add_service::ResourceAddService,
        },
        resource_list::resource_list_service::resource_list_service::ResourceListService,
        settings::settings::{COLOR_BLUE, COLOR_RED, COLOR_WHITE, EDIT_FIELD_PADDING_BOTTOM, LIST_ROW_PADDING_BOTTOM, COLUMN_LEVEL_ONE_MARGIN, COLUMN_LEVEL_TWO_MARGIN, BG_COLOR_BUTTON, COLOR_GREEN, COLOR_BLACK},
        REGISTRY,
    };

    const MIN_HEIGHT_FORM: f32 = 300.;

    pub struct ResourceListForm {
        resource_list_service: ResourceListService,
        resource_add_service: ResourceAddService,
    }

    impl ResourceListForm {
        pub fn new() -> ResourceListForm {
            ResourceListForm {
                resource_list_service: ResourceListService::new(),
                resource_add_service: ResourceAddService::new(),
            }
        }

        fn resource_list(&self, ui: &mut eframe::egui::Ui, _ctx: &Context) {
            ui.group(|ui| {
                ui.set_max_width(200.0);
                ui.set_min_width(100.0);
                ScrollArea::vertical().show(ui, |ui: &mut eframe::egui::Ui| {
                    ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                        ui.heading("Ресурсы:");
                        ui.add_space(10.);

                        let login = REGISTRY.lock().unwrap().auth_data.login.clone();

                        match self.resource_list_service.resource_list(&login) {
                            Ok(recources) => self.list(recources, ui),
                            Err(_e) => {
                                ui.colored_label(COLOR_WHITE, "Пусто".to_string());
                                ()
                            }
                        };
                    });
                });
            });
        }

        fn list(&self, list: Vec<String>, ui: &mut eframe::egui::Ui) {
            let current_resourse_while_not_clicked =
                ResourcseAddFormFacade::current_resource_name();

            for resource in list {
                let select = ui.selectable_value(
                    &mut REGISTRY
                        .lock()
                        .unwrap()
                        .form_data
                        .resource_add
                        .current_resource_name,
                    resource.clone(),
                    &resource,
                );

                if select.clicked() {
                    let login = REGISTRY.lock().unwrap().auth_data.login.clone();

                    // Если селект нажали на невыбранный ресурс
                    if current_resourse_while_not_clicked != resource {
                        ResourcseAddFormFacade::set_default_show_pass();
                        ResourcseAddFormFacade::set_decrypt_error_msg("".to_string());
                    }

                    ResourcseAddFormFacade::set_current_resource_name(resource.clone());

                    let resource = match self.resource_list_service.find_resource(&login, &resource)
                    {
                        Ok(res) => res,
                        Err(e) => {
                            ResourcseAddFormFacade::set_current_resource_login("".to_string());
                            ResourcseAddFormFacade::set_current_template_pass("".to_string());
                            ui.colored_label(COLOR_RED, e.to_string());

                            return;
                        }
                    };

                    ResourcseAddFormFacade::set_current_resource_login(resource.resource_login());

                    // расшифровка
                    match self.resource_list_service.decrypt_template_pass(&resource.template_password()) {
                        Ok(template) => {
                            ResourcseAddFormFacade::set_current_template_pass(template);
                            ResourcseAddFormFacade::set_decrypt_error_msg("".to_string());
                        }
                        Err(_) => {
                            ResourcseAddFormFacade::set_decrypt_error_msg(
                                "Невозможно расшифровать".to_string(),
                            );
                        }
                    };
                    
                    // ResourcseAddFormFacade::set_current_template_pass(resource.template_password());
                }

                if select.changed() {
                    ResourcseAddFormFacade::set_resource_field_changed(false);
                }

                ui.add_space(LIST_ROW_PADDING_BOTTOM);
                ui.add(Separator::default());
                ui.add_space(LIST_ROW_PADDING_BOTTOM);
            }
        }

        fn edit_area(&self, ui: &mut eframe::egui::Ui, ctx: &Context) {
            ui.group(|ui| {
                ui.set_min_height(MIN_HEIGHT_FORM);
                self.fields(ui, ctx);
                ui.add_space(COLUMN_LEVEL_TWO_MARGIN);
                self.controls(ui, ctx);
            });
            
        }

        fn fields(&self, ui: &mut eframe::egui::Ui, ctx: &Context) {
            ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ui.set_max_width(200.);
                    ui.add(Label::new(RichText::new("Подробно:").size(25.).color(COLOR_BLUE))
                    // .text_color(COLOR_BLUE)
                    // .heading()
                );
                });
                ui.add_space(EDIT_FIELD_PADDING_BOTTOM);

                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ui.set_max_width(200.);
                    ui.label("Ресурс:");
                    let resource_field = ui.text_edit_singleline(
                        &mut REGISTRY
                            .lock()
                            .unwrap()
                            .form_data
                            .resource_add
                            .current_resource_name,
                    );

                    if (!resource_field.changed()) {
                        return;
                    }

                    ResourcseAddFormFacade::set_resource_field_changed(true);
                });
                ui.add_space(EDIT_FIELD_PADDING_BOTTOM);

                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ui.set_max_width(200.);
                    ui.label("Логин:");
                    let login_field = ui.text_edit_singleline(
                        &mut REGISTRY
                            .lock()
                            .unwrap()
                            .form_data
                            .resource_add
                            .current_resource_login,
                    );

                    if (!login_field.changed()) {
                        return;
                    }

                    ResourcseAddFormFacade::set_resource_field_changed(true);
                });
                ui.add_space(EDIT_FIELD_PADDING_BOTTOM);

                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ui.set_max_width(200.);
                    ui.label("Пароль:");

                    if !ResourcseAddFormFacade::is_show_pass() {
                        ui.label("***");
                        return;
                    }

                    let password_field = ui.text_edit_multiline(
                        &mut REGISTRY
                            .lock()
                            .unwrap()
                            .form_data
                            .resource_add
                            .current_template_pass,
                    );

                    if (!password_field.changed()) {
                        return;
                    }

                    ResourcseAddFormFacade::set_resource_field_changed(true);
                });
            });
        }

        fn controls(&self, ui: &mut eframe::egui::Ui, ctx: &Context) {
            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                ui.horizontal_top(|ui| {
                    self.save_changes_btn(ui);
                    self.delete_btn(ui, ctx, ResourcseAddFormFacade::current_resource_name());
                    self.show_pass_btn(ui);
                });

                ui.colored_label(COLOR_RED, ResourcseAddFormFacade::decrypt_error_msg());
            });
        }

        // TODO: перенести в панель кнопку + изменение
        fn delete_btn(&self, ui: &mut eframe::egui::Ui, ctx: &Context, resource_name: String) {
            // ui.with_layout(Layout::left_to_right(), |ui| {
            let remove_btn = ui.add(
                Button::new(RichText::new("⊗").color(COLOR_RED).background_color(BG_COLOR_BUTTON))
            );
            // });

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
                        // layout_ui.spacing_mut().window_padding = Vec2::new(10., 5.);
                        layout_ui.spacing_mut().item_spacing = Vec2::new(10., 10.);
                        layout_ui.add(Label::new(format!("Точно удалить {}?", resource_name)));
                        let yes_btn = layout_ui.add(Button::new("Да")
                        // .text_color(COLOR_RED)
                    );

                        if yes_btn.clicked() {
                            let new_resource_name = ResourcseAddFormFacade::new_resource_name();
                            let new_template_name = ResourcseAddFormFacade::new_template_pass();
                            let new_resource_login = ResourcseAddFormFacade::new_resource_login();
                            let login = REGISTRY.lock().unwrap().auth_data.login.clone();

                            let resource = Resource::new(
                                new_template_name,
                                new_resource_login,
                                // "master_p".to_string(),
                                new_resource_name,
                                login,
                            );

                            match self.resource_list_service.resource_delete(resource) {
                                Ok(_) => {
                                    ResourcseAddFormFacade::set_show_confirm_delete_window(false);
                                    ResourcseAddFormFacade::set_current_resource_name("".to_string());
                                }
                                Err(e) => {
                                    ResourcseAddFormFacade::set_delete_error_msg(e.to_string());
                                }
                            };
                        }

                        let close_btn = layout_ui.add(Button::new("Нет")
                        // .text_color(COLOR_WHITE)
                    );

                        if close_btn.clicked() {
                            ResourcseAddFormFacade::set_show_confirm_delete_window(false);
                            return;
                        }
                    });

                    window_ui.label(ResourcseAddFormFacade::delete_error_msg());
                });
        }

        fn save_changes_btn(&self, ui: &mut eframe::egui::Ui) {
            let edit_btn = ui.add_enabled(
                ResourcseAddFormFacade::is_resource_field_changed(),
                Button::new(RichText::new("💾").color(COLOR_BLUE).background_color(BG_COLOR_BUTTON))
            );

            if !edit_btn.clicked() {
                return;
            }

            self.click_save_btn(ui);
        }

        fn show_pass_btn(&self, ui: &mut eframe::egui::Ui) {
            let show_pass_btn = ui.checkbox(
                &mut REGISTRY.lock().unwrap().form_data.resource_add.is_show_pass,
                "Показать пароль",
            );

            if !show_pass_btn.clicked() {
                return;
            }

            // показать пароль
            if ResourcseAddFormFacade::is_show_pass() {}

            // скрыть пароль
            if !ResourcseAddFormFacade::is_show_pass() {
                ResourcseAddFormFacade::set_default_show_pass();
                ResourcseAddFormFacade::set_decrypt_error_msg("".to_string());
            }
        }

        fn click_save_btn(&self, ui: &mut eframe::egui::Ui) {
            // TODO: можно сделать общее поля для вывода ошибок:
            // показать пароль и сохранения изменений
            ResourcseAddFormFacade::set_decrypt_error_msg("".to_string());

            let changed_resource_name = REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .current_resource_name
                .clone();
            let changed_template_pass = REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .current_template_pass
                .clone();
            let changed_resource_login = REGISTRY
                .lock()
                .unwrap()
                .form_data
                .resource_add
                .current_resource_login
                .clone();
            let login = REGISTRY.lock().unwrap().auth_data.login.clone();

            if (changed_resource_name.is_empty()
                || changed_template_pass.is_empty()
                || changed_resource_login.is_empty())
            {
                ResourcseAddFormFacade::set_decrypt_error_msg(
                    "Нужно ввести ресурс, пароль, логин".to_string(),
                )
            }

            let resource = Resource::new(
                changed_template_pass,
                changed_resource_login,
                // "master_p".to_string(),
                changed_resource_name,
                login,
            );

            match self.resource_add_service.resource_update(resource) {
                Ok(_line) => (),
                Err(e) => ResourcseAddFormFacade::set_decrypt_error_msg(e.to_string()),
            };
        }
    }

    impl Form for ResourceListForm {
        fn render(&mut self, ui: &mut eframe::egui::Ui, ctx: &Context) {
            ui.vertical(|ui: &mut eframe::egui::Ui| {
                GUI::render(ResourceAddForm::new(), ui, ctx);

                ui.add_space(20.);

                ui.horizontal(|ui: &mut eframe::egui::Ui| {
                    ui.set_min_height(MIN_HEIGHT_FORM);

                    self.resource_list(ui, ctx);

                    if ResourcseAddFormFacade::current_resource_name().is_empty() {
                        return;
                    }

                    ui.add_space(COLUMN_LEVEL_ONE_MARGIN);

                    self.edit_area(ui, ctx);
                });
            });
        }
    }
}
