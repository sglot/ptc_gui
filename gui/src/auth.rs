pub mod form;
pub mod auth_fs;
pub mod auth_service;
pub mod auth_data;


pub mod auth {
    #[derive(PartialEq, Debug)]
    pub enum Login {
        Login,
        Register,
    }

    use serde::{Serialize, Deserialize};
    
    #[derive(Serialize, Deserialize)]
    #[derive(Debug)]
    #[serde(default)]
    pub struct LastUser {
        pub login: String,
    }

impl Default for LastUser {
    fn default() -> Self {
        Self {
            login: Default::default(),
        }
    }
}

impl LastUser {
    pub fn new() -> Self {
        Self {
            login: "1".to_string(),
        }
    }
}
}

