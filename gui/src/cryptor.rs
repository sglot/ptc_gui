pub mod cryptor {
    use magic_crypt::*;

    #[derive(Debug, Clone)]
    pub struct Cryptor {
        pub key: String
    }


    impl Cryptor {
        pub fn new(key: String) -> Cryptor{
            Cryptor { key: key }
        }

        pub fn crypt(&self, data: &str) -> String {
            let mc = new_magic_crypt!(&self.key, 256);

            let base64 = mc.encrypt_str_to_base64(data);

            assert_eq!(
                data,
                mc.decrypt_base64_to_string(&base64).unwrap()
            );

            base64
        }

        pub fn decrypt(&self, data: &str) -> Result<String, String> {
            let mc = new_magic_crypt!(&self.key, 256);

            match mc.decrypt_base64_to_string(data) {
                Ok(res) => Ok(res),
                Err(e) => Err(e.to_string()),
            }
        }

        pub fn set_key(&mut self, key: String) {
            self.key = key;
        }
    }
}
