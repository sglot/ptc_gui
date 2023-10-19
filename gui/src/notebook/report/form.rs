pub mod note_report_form {
    use chrono::Utc;
    use eframe::{
        egui::{
            self, Button, Context, Label, Layout, RichText, ScrollArea, Separator, TextEdit, Window,
        },
        emath::{Align, Align2},
        epaint::Vec2,
    };
    use std::convert::TryFrom;

    use crate::{
        form::form::Form,
        note::note::note::Note,
        notebook::{
            add::{
                note_add_form_facade::note_add_form_facade::NoteAddFormFacade,
                note_add_service::note_add_service::NoteAddService,
            },
            list::note_list_service::note_list_service::NoteListService,
            report::note_report_service::note_report_service::NoteReportService,
        },
        settings::settings::{
            COLOR_BLUE, COLOR_GREEN, COLOR_RED, COLOR_WHITE, COLOR_YELLOW, DATE_FORMAT,
        },
        tag::tag_service::tag_service::TagService,
        REGISTRY,
    };

    pub struct NoteReportForm {
        note_list_service: NoteListService,
        note_report_service: NoteReportService,
        tag_service: TagService,
    }

    impl NoteReportForm {
        pub fn new() -> NoteReportForm {
            NoteReportForm {
                note_list_service: NoteListService::new(),
                tag_service: TagService::new(),
                note_report_service: NoteReportService::new(),
            }
        }

        fn note_table(&self, list: Vec<Note>, ui: &mut eframe::egui::Ui, ctx: &Context) {
            ScrollArea::vertical()
                .id_source("note_report_scroll_area")
                .stick_to_bottom()
                .auto_shrink([true; 2])
                .show(ui, |ui: &mut eframe::egui::Ui| {
                    egui::Grid::new("note_report_table").show(ui, |ui| {
                        let mut tags_summary = REGISTRY
                            .lock()
                            .unwrap()
                            .form_data
                            .note_summary
                            .typed_cache
                            .tags_cost
                            .clone();

                        tags_summary.sort_by(|a, b| a.date_start.cmp(&b.date_start));

                        for tag in &tags_summary {
                            // let tags = &REGISTRY.lock().unwrap().form_data.note_add.tags.clone();

                            ui.add(Separator::default());
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.colored_label(egui::Color32::YELLOW, format!("{}", tag.tag));
                                ui.colored_label(
                                    egui::Color32::LIGHT_BLUE,
                                    format!("~{} р.", tag.cost),
                                );
                                ui.colored_label(
                                    egui::Color32::LIGHT_GRAY,
                                    format!("({})", tag.count),
                                );

                                ui.add_space(30.);

                                ui.colored_label(
                                    egui::Color32::LIGHT_GREEN,
                                    RichText::new(format!("с {} ", tag.date_start.to_string()))
                                        .background_color(egui::Color32::BLACK),
                                );
                                ui.colored_label(
                                    egui::Color32::LIGHT_GREEN,
                                    format!("по {}", tag.date_end.to_string()),
                                );
                            });
                            // ui.label("Third row, second column");
                            ui.end_row();
                        }
                    });
                });

            ui.separator();
            ui.add_space(10.);
            ui.colored_label(
                COLOR_BLUE,
                RichText::new(
                    REGISTRY
                        .lock()
                        .unwrap()
                        .form_data
                        .note_summary
                        .typed_cache
                        .total_cost
                        .to_string(),
                )
                .italics(),
            );
        }
    }

    impl Form for NoteReportForm {
        fn render(&mut self, ui: &mut eframe::egui::Ui, ctx: &Context) {
            ui.group(|ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ui.heading(RichText::new("Отчёт:").color(egui::color::Color32::KHAKI));
                    ui.add_space(10.);

                    let login = REGISTRY.lock().unwrap().auth_data.login.clone();
                    self.note_report_service.summary(&login);

                    match self.note_list_service.note_list(&login) {
                        Ok(notes) => self.note_table(notes, ui, ctx),
                        Err(_e) => {
                            ui.colored_label(COLOR_WHITE, "Пусто".to_string());
                            ()
                        }
                    };
                });

                // TODO: переделать в ошибки для каждого поля
                ui.colored_label(COLOR_RED, NoteAddFormFacade::btn_add_error_msg());
            });
        }
    }
}
