pub mod save_fs {
    use std::fs;

    pub fn save_to_file(
        path_to: &str,
        data: &str,
        storage_file_name: &str,
    ) -> Result<String, String> {
        use std::fs::File;
        use std::io::Write;

        let mut path = String::from(path_to);
        match fs::create_dir_all(&path) {
            Ok(it) => it,
            Err(err) => return Err(err.to_string()),
        };

        path.push_str(storage_file_name);
        let mut output = match File::create(&path) {
            Ok(it) => it,
            Err(err) => return Err(err.to_string()),
        };

        match write!(output, "{}", data) {
            Ok(_) => Ok("Файл успешно сохранён".to_string()),
            Err(e) => return Err(e.to_string()),
        }
    }
}
