pub mod note_report_service {

    use chrono::NaiveDate;

    use crate::{
        note::note_repository_fs::note_repository_fs::NoteRepositoryFS,
        notebook::{
            list::note_list_service::note_list_service::NoteListService,
            report::note_summary_data::note_summary_data::TagCost,
        },
        REGISTRY,
    };

    pub struct NoteReportService {
        note_repository: NoteRepositoryFS,
        note_list_service: NoteListService,
    }

    impl NoteReportService {
        pub fn new() -> NoteReportService {
            NoteReportService {
                note_repository: NoteRepositoryFS::new(),
                note_list_service: NoteListService::new(),
            }
        }

        pub fn summary(&self, login: &str) {
            // требуется обновление кэша
            // 1. delete res
            // 2. add res
            tracing::error!("load notes");
            let is_need_calc_report = !REGISTRY
                .lock()
                .unwrap()
                .form_data
                .note_summary
                .typed_cache
                .is_set();

            if !is_need_calc_report {
                return ();
            }

            match self.note_list_service.note_list(&login) {
                Ok(res) => {
                    tracing::error!("load notes4");

                    let mut total_cost = 0.0;
                    let mut tags_summary: Vec<TagCost> = vec![];
                    for note in res {
                        total_cost = total_cost + note.cost.unwrap();

                        for tag in note.tags() {
                            let index = match tags_summary.iter().position(|x| x.tag.eq(&tag)) {
                                None => Err(()),
                                Some(pos) => Ok({ pos }),
                            };
                            tracing::error!("load notes8");
                            if index.is_err() {
                                let d = NaiveDate::parse_from_str(&note.date(), "%Y-%m-%d").unwrap();

                                tags_summary.push(TagCost {
                                    tag: tag.clone(),
                                    cost: 0.0,
                                    count: 1,
                                    date_start: d,
                                    date_end: d,
                                });

                                continue;
                            }
                            tracing::error!("load notes9");
                            tags_summary[index.unwrap()].cost += note.cost.unwrap();
                            tags_summary[index.unwrap()].count += 1;

                            let d = NaiveDate::parse_from_str(&note.date(), "%Y-%m-%d").unwrap();
                            tags_summary[index.unwrap()].date_end = d;
                            tracing::error!("load notes10");
                        }
                    }
                    tracing::error!("load notes5");

                    REGISTRY.lock().unwrap().form_data.note_summary.typed_cache.total_cost = total_cost;

                    tracing::error!("load notes6");
                    REGISTRY.lock().unwrap().form_data.note_summary.typed_cache.tags_cost = tags_summary;
                    tracing::error!("load notes7");
                }
                Err(e) => (),
            }
        }
    }
}
