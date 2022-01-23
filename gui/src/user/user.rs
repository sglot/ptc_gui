pub mod user {
    pub struct User<'a> {
        login: &'a str,
        password: &'a str,
    }

    impl User<'_> {
        pub fn new<'a>(login: &'a str, password: &'a str) -> User<'a> {
            User {login, password}
        }

        pub fn get_login(&self) -> &str {
            self.login
        }

        pub fn get_pass(&self) -> &str {
            self.password
        }
    }
}
