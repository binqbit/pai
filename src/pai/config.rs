use std::env;

use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub gpt_model: String,
    pub openai_key: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            gpt_model: String::from("gpt-3.5-turbo"),
            openai_key: None,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load() -> Option<Self> {
        let path = env::current_exe().unwrap().parent().unwrap().join("config/config.json");
        match std::fs::read_to_string(path) {
            Ok(contents) => {
                match serde_json::from_str(&contents) {
                    Ok(config) => {
                        config
                    },
                    Err(_) => {
                        println!("Failed to parse 'config.json', do you want to reset it? (Y/n)");
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        let input = input.trim().to_lowercase();
                        if input == "y" || input == "" {
                            let config = Config::default();
                            config.save();
                            Some(config)
                        } else {
                                None
                        }
                    },
                }
            },
            Err(_) => {
                Some(Config::default())
            },
        }
    }

    pub fn save(&self) {
        let path = env::current_exe().unwrap().parent().unwrap().join("config/config.json");
        if let Err(err) = std::fs::write(path, serde_json::to_string(self).unwrap()) {
            println!("Failed to save 'config.json'");
        }
    }

    pub fn set_gpt_model(&mut self, gpt_model: String) {
        self.gpt_model = gpt_model;
    }

    pub fn set_openai_key(&mut self, openai_key: String) {
        self.openai_key = Some(openai_key);
    }
}