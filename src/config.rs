use serde::{Deserialize, Serialize};
use std::fs;

const CONFIG_FILE: &str = "config.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub cancel_keyword: String,
}

impl Config {
    pub fn default() -> Self {
        Config {
            cancel_keyword: "cancel".to_string(),
        }
    }

    pub fn load() -> Self {
        if let Ok(data) = fs::read_to_string(CONFIG_FILE) {
            serde_json::from_str(&data).unwrap_or_else(|_| Config::default())
        } else {
            Config::default()
        }
    }

    pub fn save(&self) {
        let data = serde_json::to_string_pretty(self).expect("Failed to serialize config");
        fs::write(CONFIG_FILE, data).expect("Failed to write config file");
    }
}
