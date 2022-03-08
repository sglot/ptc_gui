pub mod resource {
    pub struct Resource {
        template_password: String,
        resource_login: String,
        // master_password: String,
        // 
        name: String,
        user_login: String,
    }

    impl Resource {
        pub fn new<'a>(template_password: String, resource_login: String, name: String, user_login: String, ) -> Resource {
            Resource {name, template_password, user_login, resource_login}
        }

        pub fn name(&self) -> String {
            self.name.clone()
        }

        pub fn template_password(&self) -> String {
            self.template_password.clone()
        }

        pub fn resource_login(&self) -> String {
            self.resource_login.clone()
        }

        pub fn user_login(&self) -> String {
            self.user_login.clone()
        }
    }
}
