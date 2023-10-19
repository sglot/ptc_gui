pub mod form {
    use eframe::{
        egui::{self, Button, Context, Label, Layout, RichText, ScrollArea, Separator, Window, Id},
        emath::{Align, Align2},
        epaint::Vec2,
    };

    use crate::{
        form::{form::Form, main_form::main_form::GUI},
        note::note::note::Note,
        notebook::{
            add::{
                form::note_add_form::NoteAddForm,
                note_add_form_facade::note_add_form_facade::NoteAddFormFacade,
            },
            list::note_list_service::note_list_service::NoteListService, report::form::note_report_form::NoteReportForm,
        },
        settings::settings::{COLOR_WHITE, COLOR_YELLOW, LIST_ROW_PADDING_BOTTOM},
        REGISTRY,
    };
    pub const ROW_PADDING: f32 = 5.0;
    pub const TOP_PADDING: f32 = 30.0;

    pub struct NotebookForm {
        note_list_service: NoteListService,
    }

    impl NotebookForm {
        pub fn new() -> NotebookForm {
            NotebookForm {
                note_list_service: NoteListService::new(),
            }
        }

        fn list(&self, ui: &mut eframe::egui::Ui, ctx: &Context) {
            ui.group(|ui| {
                ui.set_max_width(500.);
                ui.set_min_height(600.);
                ui.heading(RichText::new("Заметки:").color(egui::color::Color32::KHAKI));
                ui.add_space(20.);

                ScrollArea::vertical()
                    .stick_to_bottom()
                    .auto_shrink([true; 2])
                    .show(ui, |ui: &mut eframe::egui::Ui| {
                        ui.spacing_mut().item_spacing = Vec2::new(10., 10.);

                        let login = REGISTRY.lock().unwrap().auth_data.login.clone();

                        match self.note_list_service.note_list(&login) {
                            Ok(notes) => self.note_list(notes, ui, ctx),
                            Err(_e) => {
                                ui.colored_label(COLOR_WHITE, "Пусто".to_string());
                                ()
                            }
                        };
                    });
            });
        }

        fn note_list(&self, list: Vec<Note>, ui: &mut eframe::egui::Ui, ctx: &Context) {
            for note in list {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(10., 10.);

                    //head
                    ui.horizontal(|ui| {
                        let btn_edit = ui.add(Button::new(
                            RichText::new(format!("#{}", note.id().to_string()))
                                .color(COLOR_YELLOW),
                        ));

                        if btn_edit.clicked_by(egui::PointerButton::Primary) {
                            self.note_list_service.load_for_update_from_list(&note);
                        }

                        if btn_edit.clicked_by(egui::PointerButton::Secondary) {
                            NoteAddFormFacade::set_show_confirm_delete_note_window(true);
                            NoteAddFormFacade::set_note_for_delete(note.id());
                        }

                        if NoteAddFormFacade::show_confirm_delete_note_window() {
                            self.delete_window(ui, ctx);
                        }

                        ui.horizontal(|ui| {
                            ui.set_width(100.);
                            ui.text_edit_singleline(&mut note.date());
                        });

                        ui.label(
                            RichText::new(note.title())
                                .color(egui::Color32::from_rgb(177, 177,177))
                                .strong(),
                        );
                    });

                    //data
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

                    // tags
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

                    // text
                    if !note.text.is_empty() {
                        ui.horizontal(|ui| {
                            ui.set_min_width(300.);
                            ui.style_mut().visuals.extreme_bg_color = egui::Color32::from_rgb(47, 47, 47);
                            ui.style_mut().visuals.override_text_color =
                                Option::Some(egui::Color32::WHITE);
                            ui.text_edit_multiline(&mut note.text());
                        });
                        ui.add_space(10.);
                    }
                });

                ui.add_space(LIST_ROW_PADDING_BOTTOM);
                ui.add(Separator::default());
                ui.add_space(LIST_ROW_PADDING_BOTTOM);
            }
        }

        fn delete_window(&self, _ui: &mut eframe::egui::Ui, ctx: &Context) {
            Window::new("Удаление заметки")
            .id(egui::Id::new("window_note"))
                .anchor(Align2::CENTER_CENTER, Vec2::new(0., 0.))
                .show(ctx, |window_ui| {
                    window_ui.set_height(50.);
                    window_ui.set_row_height(100.);

                    window_ui.with_layout(Layout::top_down_justified(Align::Center), |layout_ui| {
                        layout_ui.add_space(10.);
                        layout_ui.spacing_mut().button_padding = Vec2::new(10., 5.);
                        // layout_ui.spacing_mut().window_padding = Vec2::new(10., 5.);
                        layout_ui.spacing_mut().item_spacing = Vec2::new(10., 10.);
                        layout_ui.add(Label::new(format!("Точно удалить {}?", "заметку")));

                        let yes_btn = layout_ui.add(
                            Button::new("Да"), // .text_color(COLOR_RED)
                        );

                        if yes_btn.clicked() {
                            tracing::error!("---------------------------------------------------1212212313131312331231313123123");
                            match self.note_list_service.delete(NoteAddFormFacade::note_for_delete()) {
                                Ok(_) => {
                                    NoteAddFormFacade::set_show_confirm_delete_note_window(false);
                                }
                                Err(e) => {
                                    NoteAddFormFacade::set_delete_error_msg(e.to_string());
                                    NoteAddFormFacade::set_show_confirm_delete_note_window(false);
                                }
                            };
                            NoteAddFormFacade::set_note_for_delete(0);
                        }

                        let close_btn = layout_ui.add(Button::new("Нет"));

                        if close_btn.clicked() {
                            NoteAddFormFacade::set_show_confirm_delete_note_window(false);
                            NoteAddFormFacade::set_note_for_delete(0);
                            return;
                        }
                    });

                    window_ui.label(NoteAddFormFacade::delete_error_msg());
                });
        }
    }

    impl Form for NotebookForm {
        fn render(&mut self, ui: &mut eframe::egui::Ui, ctx: &Context) {
            ui.with_layout(
                Layout::top_down_justified(Align::Min),
                |ui: &mut eframe::egui::Ui| {
                    ui.horizontal(|ui: &mut eframe::egui::Ui| {
                        GUI::render(NoteAddForm::new(), ui, ctx);

                        ui.add_space(20.);

                        ui.with_layout(
                            Layout::top_down(Align::LEFT),
                            |ui: &mut eframe::egui::Ui| {
                                // ui.set_min_height(MIN_HEIGHT_FORM);
                                // ширина рамки
                                // ui.set_width(MIN_HEIGHT_FORM);

                                self.list(ui, ctx);
                            },
                        );

                        ui.add_space(20.);

                        GUI::render(NoteReportForm::new(), ui, ctx);
                    });
                },
            );
        }
    }
}
//                 use egui::plot::{Line, Plot, Value, Values};
// let sin = (0..1000).map(|i| {
//     let x = i as f64 * 0.01;
//     Value::new(x, x.sin())
// });
// let line = Line::new(Values::from_values_iter(sin));
// Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| plot_ui.line(line));
