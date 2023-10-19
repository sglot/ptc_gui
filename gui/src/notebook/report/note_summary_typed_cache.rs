pub mod note_summary_typed_cache {
    use crate::{note::note::note::Note, notebook::report::note_summary_data::note_summary_data::TagCost};

    pub struct NoteSummaryTypedCache {
        pub total_cost: f32,
        pub tags_cost: Vec<TagCost>,
    }

    impl NoteSummaryTypedCache {
        pub fn new() -> Self {
            Self {
                total_cost: 0.0,
                tags_cost: vec![],
            }
        }

        pub fn get(&self) -> Vec<TagCost> {
            self.tags_cost.clone()
        }

        pub fn set(&mut self, tags_cost: Vec<TagCost>) {
            self.tags_cost = tags_cost;
        }

        pub fn drop(&mut self) {
            self.tags_cost = vec![];
        }

        pub fn is_set(&mut self) -> bool {
            !self.tags_cost.is_empty()
        }
    }
}
