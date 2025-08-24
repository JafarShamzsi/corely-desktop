use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app_data_dir: PathBuf,
    pub models_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub log_level: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // TODO: Load from config file or create default
        let app_data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ai-training-desktop");
        
        Ok(Self {
            models_dir: app_data_dir.join("models"),
            cache_dir: app_data_dir.join("cache"),
            app_data_dir,
            log_level: "info".to_string(),
        })
    }
    
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Save config to file
        Ok(())
    }
}
