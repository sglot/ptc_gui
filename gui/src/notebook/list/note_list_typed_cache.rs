pub mod note_list_typed_cache {
    use crate::note::note::note::Note;

    pub struct NoteListTypedCache {
        pub note_list: Vec<Note>,
    }

    impl NoteListTypedCache {
        pub fn new() -> Self {
            Self {
                note_list: vec![],
            }
        }

        pub fn note_list(&self) -> Vec<Note> {
            self.note_list.clone()
        }

        pub fn set_note_list(&mut self, note_list: Vec<Note>) {
            self.note_list = note_list;
        }

        pub fn drop_note_list(&mut self) {
            self.note_list = vec![];
        }
    }
}
