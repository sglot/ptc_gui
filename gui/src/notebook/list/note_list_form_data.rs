pub mod note_list_form_data {
    use crate::notebook::list::note_list_typed_cache::note_list_typed_cache::NoteListTypedCache;

    pub struct NoteListFormData {
        pub typed_cache: NoteListTypedCache,
    }

    impl NoteListFormData {
        pub fn new() -> Self {
            Self {
                typed_cache: NoteListTypedCache::new(),
            }
        }
    }
}