pub mod resource {
    pub struct Resource {
        name: String,
        template: String,
        login: String,
    }

    impl Resource {
        pub fn new<'a>(name: String, template: String, login: String) -> Resource {
            Resource {name, template, login}
        }

        pub fn name(&self) -> String {
            self.name.clone()
        }

        pub fn template(&self) -> String {
            self.template.clone()
        }

        pub fn login(&self) -> String {
            self.login.clone()
        }
    }
}
