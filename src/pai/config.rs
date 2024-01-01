use serde::{Serialize, Deserialize};

use crate::get_exec_path;



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub apikey: String,
}


impl Config {
    pub fn load() -> Option<Self> {
        let path = get_exec_path().join("config/config.json");
        match std::fs::read_to_string(path) {
            Ok(contents) => {
                match serde_json::from_str(&contents) {
                    Ok(config) => {
                        Some(config)
                    },
                    Err(_) => {
                        println!("Failed to parse 'config.json'");
                        None
                    },
                }
            },
            Err(_) => {
                println!("Failed to load 'config.json'");
                None
            },
        }
    }

    pub fn save(&self) {
        let path = get_exec_path().join("config/config.json");
        if std::fs::write(path, serde_json::to_string(self).unwrap()).is_err() {
            println!("Failed to save 'config.json'");
        }
    }

    pub fn set_apikey(apikey: String) -> Self {
        let config = Self {
            apikey,
        };
        config.save();
        config
    }
}