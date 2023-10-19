pub mod note_add_form {
    use chrono::Utc;
    use eframe::{
        egui::{self, Button, Context, Label, Layout, RichText, ScrollArea, TextEdit, Window},
        emath::{Align, Align2},
        epaint::Vec2,
    };
    use std::convert::TryFrom;

    use crate::{
        form::form::Form,
        note::note::note::Note,
        notebook::add::{
            note_add_form_facade::note_add_form_facade::NoteAddFormFacade,
            note_add_service::note_add_service::NoteAddService,
        },
        settings::settings::{
            COLOR_BLACK, COLOR_BLUE, COLOR_GREEN, COLOR_RED, COLOR_WHITE, COLOR_YELLOW, DATE_FORMAT,
        },
        tag::tag_service::tag_service::TagService,
        REGISTRY,
    };

    pub struct NoteAddForm {
        note_add_service: NoteAddService,
        tag_service: TagService,
    }

    impl NoteAddForm {
        pub fn new() -> NoteAddForm {
            NoteAddForm {
                note_add_service: NoteAddService::new(),
                tag_service: TagService::new(),
            }
        }

        fn click_btn_save(&self, _ui: &mut eframe::egui::Ui) {
            NoteAddFormFacade::set_add_error_msg("".to_string());

            let last_notes = REGISTRY
                .lock()
                .unwrap()
                .form_data
                .note_list
                .typed_cache
                .note_list();
            let last_note = last_notes.last();

            let mut is_update = false;

            let new_id = match &last_note {
                Some(note) => {
                    if REGISTRY.lock().unwrap().form_data.note_add.id == 0 {
                        note.id() + 1
                    } else {
                        is_update = true;
                        REGISTRY.lock().unwrap().form_data.note_add.id
                    }
                }
                None => 1,
            };

            let new_title = REGISTRY.lock().unwrap().form_data.note_add.title.clone();
            let mut new_date = REGISTRY.lock().unwrap().form_data.note_add.date.clone();
            let new_text = REGISTRY.lock().unwrap().form_data.note_add.text.clone();
            let new_mileage = REGISTRY.lock().unwrap().form_data.note_add.mileage.clone();
            let new_cost = REGISTRY.lock().unwrap().form_data.note_add.cost.clone();
            let new_tags = REGISTRY.lock().unwrap().form_data.note_add.tags.clone();
            let login = REGISTRY.lock().unwrap().auth_data.login.clone();

            // if (new_resource_name.is_empty() || new_template_pass.is_empty() || new_resource_login.is_empty()) {
            //     NoteAddFormFacade::set_add_error_msg("Нужно ввести ресурс, пароль, логин".to_string())
            // }

            if new_date == "сегодня" {
                new_date = Utc::now().format(DATE_FORMAT).to_string();
            }

            let note = Note::new(
                new_id,
                login,
                new_date,
                new_title,
                new_text,
                Option::Some(new_mileage.parse::<u32>().unwrap()),
                Option::Some(new_cost.parse::<f32>().unwrap()),
                new_tags,
            );

            match is_update {
                true => match self.note_add_service.update(note) {
                    Ok(_line) => (),
                    Err(e) => NoteAddFormFacade::set_add_error_msg(e.to_string()),
                },
                false => match self.note_add_service.add(note) {
                    Ok(_line) => (),
                    Err(e) => NoteAddFormFacade::set_add_error_msg(e.to_string()),
                },
            }
        }

        fn tag_grid(&self, ui: &mut eframe::egui::Ui, ctx: &Context) {
            egui::Grid::new("tag_grid").show(ui, |ui| {
                ui.set_min_height(200.);

                let mut count: i32 = 0;
                let tag_list = REGISTRY.lock().unwrap().form_data.note_add.tag_list.clone();

                let tag_for_delete = NoteAddFormFacade::tag_for_delete();

                for tag in &tag_list {
                    let mut can_delete = true;
                    count += 1;

                    let tags = &REGISTRY.lock().unwrap().form_data.note_add.tags.clone();

                    let index = tags.iter().position(|x| x.eq(tag));
                    let tag_btn = match index {
                        Some(_) => {
                            can_delete = false;
                            ui.add(Button::new(RichText::new(tag).color(COLOR_YELLOW)))
                        }
                        None => {
                            let mut color = COLOR_WHITE;
                            if tag.eq(&tag_for_delete) {
                                color = COLOR_RED;
                            };

                            ui.add(Button::new(RichText::new(tag).color(color)))
                        }
                    };

                    if tag_btn.clicked() {
                        match index {
                            Some(i) => {
                                REGISTRY
                                    .lock()
                                    .unwrap()
                                    .form_data
                                    .note_add
                                    .tags
                                    .remove(usize::try_from(i).unwrap());
                                ()
                            }
                            None => REGISTRY
                                .lock()
                                .unwrap()
                                .form_data
                                .note_add
                                .tags
                                .push(tag.clone()),
                        };
                    }

                    if tag_btn.clicked_by(egui::PointerButton::Secondary) {
                        if can_delete {
                            NoteAddFormFacade::set_show_confirm_delete_window(true);
                            NoteAddFormFacade::set_tag_for_delete(tag.clone());
                        }
                    }

                    if NoteAddFormFacade::show_confirm_delete_window() {
                        self.delete_window(ui, ctx);
                    }

                    if count % 4 == 0 {
                        ui.end_row();
                    }
                }
            });
        }

        fn create_tag(&self, ui: &mut eframe::egui::Ui) {
            ui.with_layout(
                egui::Layout::left_to_right().with_cross_justify(true),
                |ui| {
                    ui.add(TextEdit::singleline(
                        &mut REGISTRY.lock().unwrap().form_data.note_add.create_tag,
                    ));

                    let add_btn = ui.add(Button::new(RichText::new("+тэг").color(COLOR_GREEN)));

                    if !add_btn.clicked() {
                        return;
                    }

                    let new_tag = REGISTRY
                        .lock()
                        .unwrap()
                        .form_data
                        .note_add
                        .create_tag
                        .clone();

                    if new_tag.is_empty() {
                        return;
                    }
                    tracing::error!("1");
                    match self.tag_service.add(new_tag.clone()) {
                        Ok(_line) => (),
                        Err(e) => NoteAddFormFacade::set_add_error_msg(e.to_string()),
                    };

                    tracing::error!("2");
                    NoteAddFormFacade::drop_create_tag();
                },
            );
        }

        // TODO: перенести в панель кнопку + изменение
        fn delete_window(&self, _ui: &mut eframe::egui::Ui, ctx: &Context) {
            // ui.with_layout(Layout::left_to_right(), |ui| {
            // let remove_btn = ui.add(
            //     Button::new(RichText::new("⊗").color(COLOR_RED).background_color(BG_COLOR_BUTTON))
            // );
            // // });

            // if remove_btn.clicked() {
            //     ResourcseAddFormFacade::set_show_confirm_delete_window(true);
            //     return;
            // }

            // if !ResourcseAddFormFacade::show_confirm_delete_window() {
            //     return;
            // }

            Window::new("Удаление")
                .id(egui::Id::new("window_tag"))
                .anchor(Align2::CENTER_CENTER, Vec2::new(0., 0.))
                .show(ctx, |window_ui| {
                    window_ui.set_height(50.);
                    window_ui.set_row_height(100.);

                    window_ui.with_layout(Layout::top_down_justified(Align::Center), |layout_ui| {
                        layout_ui.add_space(10.);
                        layout_ui.spacing_mut().button_padding = Vec2::new(10., 5.);
                        // layout_ui.spacing_mut().window_padding = Vec2::new(10., 5.);
                        layout_ui.spacing_mut().item_spacing = Vec2::new(10., 10.);
                        layout_ui.add(Label::new(format!("Точно удалить {}?", "Тэг")));
                        let yes_btn = layout_ui.add(
                            Button::new("Да"), // .text_color(COLOR_RED)
                        );

                        if yes_btn.clicked() {
                            // let new_resource_name = ResourcseAddFormFacade::new_resource_name();
                            // let new_template_name = ResourcseAddFormFacade::new_template_pass();
                            // let new_resource_login = ResourcseAddFormFacade::new_resource_login();
                            // let login = REGISTRY.lock().unwrap().auth_data.login.clone();

                            // let resource = Resource::new(
                            //     new_template_name,
                            //     new_resource_login,
                            //     // "master_p".to_string(),
                            //     new_resource_name,
                            //     login,
                            // );

                            // удаление тэга в общей переменной ошибки для формы
                            match self.tag_service.delete(NoteAddFormFacade::tag_for_delete()) {
                                Ok(_) => {
                                    NoteAddFormFacade::set_show_confirm_delete_window(false);
                                }
                                Err(e) => {
                                    NoteAddFormFacade::set_delete_error_msg(e.to_string());
                                    NoteAddFormFacade::set_show_confirm_delete_window(false);
                                }
                            };
                            NoteAddFormFacade::set_tag_for_delete("".to_string());
                        }

                        let close_btn = layout_ui.add(
                            Button::new("Нет"), // .text_color(COLOR_WHITE)
                        );

                        if close_btn.clicked() {
                            NoteAddFormFacade::set_show_confirm_delete_window(false);
                            NoteAddFormFacade::set_tag_for_delete("".to_string());
                            return;
                        }
                    });

                    window_ui.label(NoteAddFormFacade::delete_error_msg());
                });
        }
    }

    impl Form for NoteAddForm {
        fn render(&mut self, ui: &mut eframe::egui::Ui, ctx: &Context) {
            ui.group(|ui| {
                ui.set_min_height(50.);
                ui.set_max_width(150.);

                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ui.heading(RichText::new("Новая заметка:").color(egui::color::Color32::KHAKI));
                    ui.add_space(10.);

                    ui.separator();

                    ui.with_layout(egui::Layout::left_to_right(), |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                            ui.set_max_width(100.);
                            ui.label("Дата:");
                            ui.text_edit_singleline(
                                &mut REGISTRY.lock().unwrap().form_data.note_add.date,
                            );
                        });

                        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                            ui.set_max_width(150.);
                            ui.label("Название:");
                            ui.text_edit_singleline(
                                &mut REGISTRY.lock().unwrap().form_data.note_add.title,
                            );
                        });

                        let refresh_btn = ui.add(Button::new("🔄"));

                        if !refresh_btn.clicked() {
                            return;
                        }

                        NoteAddFormFacade::set_default();
                    });

                    ui.add_space(10.);
                    ui.colored_label(COLOR_BLUE, "Спец. данные");

                    ui.with_layout(egui::Layout::left_to_right(), |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                            ui.set_max_width(100.);
                            ui.label("Пробег:");
                            ui.text_edit_singleline(
                                &mut REGISTRY.lock().unwrap().form_data.note_add.mileage,
                            );
                        });

                        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                            ui.set_max_width(150.);
                            ui.label("Сумма (руб.):");
                            ui.text_edit_singleline(
                                &mut REGISTRY.lock().unwrap().form_data.note_add.cost,
                            );
                        });
                    });

                    ui.add_space(10.);
                    ui.colored_label(COLOR_BLUE, "Комментарий");
                    ui.add(TextEdit::multiline(
                        &mut REGISTRY.lock().unwrap().form_data.note_add.text,
                    ));

                    ui.add_space(20.);

                    ui.separator();

                    // ui.set_min_width(300.0);
                    // ui.set_min_height(200.0);
                    self.create_tag(ui);

                    ui.add_space(10.);

                    ScrollArea::vertical().id_source("tag_scroll_area").show(
                        ui,
                        |ui: &mut eframe::egui::Ui| {
                            self.tag_grid(ui, ctx);
                        },
                    );

                    ui.add_space(20.);

                    ui.separator();

                    ui.with_layout(
                        egui::Layout::top_down(egui::Align::TOP),
                        |ui| {
                            ui.set_width(100.);
                            let btn_save =
                                ui.add(Button::new(RichText::new("Сохранить").color(COLOR_GREEN)));
    
                            // let btn_save =
                            // ui.add(Label::new(RichText::new("Сохранить").strong().underline().background_color(COLOR_BLACK).color(COLOR_GREEN)));
    
                            if btn_save.clicked() {
                                self.click_btn_save(ui);
                            }
                        },
                    );
                });

                

                // TODO: переделать в ошибки для каждого поля
                ui.colored_label(COLOR_RED, NoteAddFormFacade::btn_add_error_msg());
            });
        }
    }
}
