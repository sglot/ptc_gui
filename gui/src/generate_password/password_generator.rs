pub mod password_generator {
    #[derive(Debug, Clone)]
    pub struct PasswordGenerator {
        pub chars_count: String,
    }

    impl PasswordGenerator {
        pub fn new(chars_count: String) -> PasswordGenerator {
            PasswordGenerator {
                chars_count: chars_count,
            }
        }

        pub fn generate(&self) -> String {
            use rand::Rng;
            const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
            let PASSWORD_LEN: i32 = self.chars_count.parse::<i32>().unwrap();
            let mut rng = rand::thread_rng();

            let password: String = (0..PASSWORD_LEN)
                .map(|_| {
                    let idx = rng.gen_range(0..CHARSET.len());
                    CHARSET[idx] as char
                })
                .collect::<String>();
                
                password
        }

        pub fn set_chars_count(&mut self, chars_count: String) {
            self.chars_count = chars_count;
        }
    }
}
