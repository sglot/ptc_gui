pub mod note {
    use serde::{Deserialize, Serialize};

    #[derive(Default)]
    #[derive(Clone)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Note {
        id: u64,
        pub user: String,
        pub date: String,
        pub title: String,
        pub text: String,
        pub mileage: Option<u32>,
        pub cost: Option<f32>,
        pub tags: Vec<String>,
    }

    impl Note {
        pub fn new<'a>(id: u64, user: String, date: String, title: String, text: String, mileage: Option<u32>, cost: Option<f32>, tags: Vec<String>) -> Note {
            Note {id, user, date, title, text, mileage, cost, tags}
        }

        pub fn id(&self) -> u64 {
            self.id
        }

        pub fn date(&self) -> String {
            self.date.clone()
        }

        pub fn title(&self) -> String {
            self.title.clone()
        }

        pub fn text(&self) -> String {
            self.text.clone()
        }

        pub fn mileage(&self) -> Option<u32> {
            self.mileage.clone()
        }

        pub fn cost(&self) -> Option<f32> {
            self.cost.clone()
        }

        pub fn user(&self) -> String {
            self.user.clone()
        }

        pub fn tags(&self) -> Vec<String> {
            self.tags.clone()
        }
    }
}
