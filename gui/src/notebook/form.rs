pub mod form {
    use eframe::{
        egui::{
            self,
            Context,
            Layout,
            ScrollArea,
            Separator, RichText,
        },
        emath::Align,
        epaint::{Vec2},
    };

    use crate::{
        form::{
            form::{Form}, main_form::main_form::GUI,
        },
        note::note::note::Note,
        notebook::{
            add::form::note_add_form::NoteAddForm,
            list::note_list_service::note_list_service::NoteListService,
        },
        settings::settings::{
            COLOR_BLUE, COLOR_WHITE, COLOR_YELLOW,
            LIST_ROW_PADDING_BOTTOM,
        },
        REGISTRY,
    };
    pub const ROW_PADDING: f32 = 5.0;
    pub const TOP_PADDING: f32 = 30.0;
    const MIN_HEIGHT_FORM: f32 = 300.;

    pub struct NotebookForm {
        note_list_service: NoteListService,
    }

    impl NotebookForm {
        pub fn new() -> NotebookForm {
            NotebookForm {
                note_list_service: NoteListService::new(),
            }
        }

        fn list(&self, ui: &mut eframe::egui::Ui, _ctx: &Context) {
            ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                ui.group(|ui| {
                    // ui.set_max_width(300.0);
                    ui.set_min_width(300.0);
                    ui.set_min_height(550.0);
                    // ui.spacing_mut().item_spacing = Vec2::new(10., 10.);
                    ScrollArea::vertical().show(ui, |ui: &mut eframe::egui::Ui| {
                        ui.spacing_mut().item_spacing = Vec2::new(10., 10.);

                        // ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                        ui.heading("Заметки:");
                        // ui.add_space(10.);

                        let login = REGISTRY.lock().unwrap().auth_data.login.clone();

                        match self.note_list_service.note_list(&login) {
                            Ok(notes) => self.note_list(notes, ui),
                            Err(_e) => {
                                ui.colored_label(COLOR_WHITE, "Пусто".to_string());
                                ()
                            }
                        };
                        // });
                    });
                });
            });
        }

        fn note_list(&self, list: Vec<Note>, ui: &mut eframe::egui::Ui) {
            // let current_resourse_while_not_clicked =
            //     ResourcseAddFormFacade::current_resource_name();
            for note in list {
                ui.vertical(|ui| {
                    
                    ui.spacing_mut().item_spacing = Vec2::new(10., 10.);

                    //     ui.spacing_mut().item_spacing = Vec2::new(1., 1.);

                    ui.horizontal(|ui| {
                        let id_label =
                            ui.colored_label(COLOR_WHITE, format!("#{}", note.id().to_string()));

                        let date_label =
                        ui.label(RichText::new(note.date()).color(egui::Color32::LIGHT_BLUE).monospace());
                        let title =
                        // ui.add(egui::Label::new(note.title()).text_color(egui::Color32::GRAY));
                        ui.label(RichText::new(note.title()).color(egui::Color32::DARK_GRAY).strong());
                    });

                    ui.horizontal(|ui| {
                        match note.mileage() {
                            Some(km) => {
                                ui.colored_label(COLOR_WHITE, format!("{:?}км", &km));
                                ()
                            }
                            None => (),
                        }

                        match note.cost() {
                            Some(rub) => {
                                ui.colored_label(COLOR_WHITE, format!("{:?}р", &rub));
                                ()
                            }
                            None => (),
                        }
                    });

                    if !note.tags.is_empty() {
                        egui::Grid::new("grid2").show(ui, |ui| {
                            let mut count = 0;
                            for tag in &note.tags() {
                                count += 1;

                                ui.colored_label(egui::Color32::GRAY, format!("{}", tag));

                                if count % 5 == 0 {
                                    ui.end_row();
                                }
                            }
                        });
                    }

                    if !note.text.is_empty() {
                        ui.add_space(10.);
                        ui.colored_label(COLOR_WHITE, format!("{}", note.text()));
                    }
                });

                // if select.clicked() {
                //     let login = REGISTRY.lock().unwrap().auth_data.login.clone();

                //     // Если селект нажали на невыбранный ресурс
                //     if current_resourse_while_not_clicked != resource {
                //         ResourcseAddFormFacade::set_default_show_pass();
                //         ResourcseAddFormFacade::set_decrypt_error_msg("".to_string());
                //     }

                //     ResourcseAddFormFacade::set_current_resource_name(resource.clone());

                //     let resource = match self.resource_list_service.find_resource(&login, &resource)
                //     {
                //         Ok(res) => res,
                //         Err(e) => {
                //             ResourcseAddFormFacade::set_current_resource_login("".to_string());
                //             ResourcseAddFormFacade::set_current_template_pass("".to_string());
                //             ui.colored_label(COLOR_RED, e.to_string());

                //             return;
                //         }
                //     };

                //     ResourcseAddFormFacade::set_current_resource_login(resource.resource_login());

                //     // расшифровка
                //     match self.resource_list_service.decrypt_template_pass(&resource.template_password()) {
                //         Ok(template) => {
                //             ResourcseAddFormFacade::set_current_template_pass(template);
                //             ResourcseAddFormFacade::set_decrypt_error_msg("".to_string());
                //         }
                //         Err(_) => {
                //             ResourcseAddFormFacade::set_decrypt_error_msg(
                //                 "Невозможно расшифровать".to_string(),
                //             );
                //         }
                //     };

                //     // ResourcseAddFormFacade::set_current_template_pass(resource.template_password());
                // }

                // if select.changed() {
                //     ResourcseAddFormFacade::set_resource_field_changed(false);
                // }

                ui.add_space(LIST_ROW_PADDING_BOTTOM);
                ui.add(Separator::default());
                ui.add_space(LIST_ROW_PADDING_BOTTOM);
            }
        }
    }

    impl Form for NotebookForm {
        fn render(&mut self, ui: &mut eframe::egui::Ui, ctx: &Context) {
            ui.horizontal_top(|ui: &mut eframe::egui::Ui| {
                GUI::render(NoteAddForm::new(), ui, ctx);

                ui.add_space(20.);

                ui.with_layout(
                    Layout::top_down(Align::LEFT),
                    |ui: &mut eframe::egui::Ui| {
                        ui.set_min_height(MIN_HEIGHT_FORM);
                        ui.set_width(MIN_HEIGHT_FORM);

                        self.list(ui, ctx);
                    },
                );
            });
        }
    }
}
