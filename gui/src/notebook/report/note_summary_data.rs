pub mod note_summary_data {
    use chrono::NaiveDate;

    use crate::notebook::report::note_summary_typed_cache::note_summary_typed_cache::NoteSummaryTypedCache;

    pub struct NoteSummaryData {
        
        pub typed_cache: NoteSummaryTypedCache,
    }

    #[derive(Clone)]
    pub struct TagCost {
        pub tag: String,
        pub cost: f32,
        pub count: u32,
        pub date_start: NaiveDate,
        pub date_end: NaiveDate,
    }

    impl NoteSummaryData {
        pub fn new() -> Self {
            Self {
                
                typed_cache: NoteSummaryTypedCache::new(),
            }
        }
    }
}