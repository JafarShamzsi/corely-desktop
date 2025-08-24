use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use chrono;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub user_id: Option<String>,
    pub api_endpoint: String,
    pub enable_distributed_training: bool,
    pub max_memory_usage_gb: u64,
    pub preferred_gpu: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            user_id: None,
            api_endpoint: "https://api.ai-training.com".to_string(),
            enable_distributed_training: false,
            max_memory_usage_gb: 8,
            preferred_gpu: None,
        }
    }
}

pub struct AppState {
    pub config: Mutex<AppConfig>,
    pub is_training: Mutex<bool>,
    pub current_model: Mutex<Option<String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            config: Mutex::new(AppConfig::default()),
            is_training: Mutex::new(false),
            current_model: Mutex::new(None),
        }
    }
}
