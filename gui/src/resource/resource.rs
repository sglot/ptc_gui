pub mod resource {
    pub struct Resource<'a> {
        name: &'a str,
    }

    impl Resource<'_> {
        pub fn new<'a>(name: &'a str) -> Resource<'a> {
            Resource {name}
        }

        pub fn name(&self) -> &str {
            self.name
        }
    }
}
