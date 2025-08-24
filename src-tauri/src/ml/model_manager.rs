use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use chrono;
use crate::ml::candle_integration::CandleManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub framework: String,
    pub version: String,
    pub file_path: PathBuf,
    pub size_bytes: u64,
    pub parameters: u64,
    pub input_shape: Vec<usize>,
    pub output_shape: Vec<usize>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelStatus {
    pub loaded: bool,
    pub memory_usage_mb: Option<u64>,
    pub last_used: Option<String>,
    pub performance_metrics: Option<PerformanceMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub inference_time_ms: f64,
    pub throughput_samples_per_sec: f64,
    pub memory_peak_mb: u64,
}

pub struct ModelManager {
    models: HashMap<String, ModelMetadata>,
    model_status: HashMap<String, ModelStatus>,
    candle_manager: Option<CandleManager>,
    models_directory: PathBuf,
}

impl ModelManager {
    pub fn new(models_directory: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let candle_manager = CandleManager::new().ok();
        
        Ok(Self {
            models: HashMap::new(),
            model_status: HashMap::new(),
            candle_manager,
            models_directory,
        })
    }
    
    pub fn scan_models_directory(&mut self) -> Result<Vec<ModelMetadata>, Box<dyn std::error::Error>> {
        let mut discovered_models = Vec::new();
        
        if !self.models_directory.exists() {
            std::fs::create_dir_all(&self.models_directory)?;
            return Ok(discovered_models);
        }
        
        // TODO: Implement model file scanning and metadata extraction
        // This would scan for .safetensors, .onnx, .pt files etc.
        
        Ok(discovered_models)
    }
    
    pub fn register_model(&mut self, metadata: ModelMetadata) {
        let model_id = metadata.id.clone();
        self.models.insert(model_id.clone(), metadata);
        self.model_status.insert(model_id, ModelStatus {
            loaded: false,
            memory_usage_mb: None,
            last_used: None,
            performance_metrics: None,
        });
    }
    
    pub fn load_model(&mut self, model_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        if let Some(metadata) = self.models.get(model_id) {
            if let Some(ref mut candle_manager) = self.candle_manager {
                candle_manager.load_model(model_id, &metadata.file_path)?;
                
                if let Some(status) = self.model_status.get_mut(model_id) {
                    status.loaded = true;
                    status.last_used = Some(chrono::Utc::now().to_rfc3339());
                }
                
                return Ok(true);
            }
        }
        Ok(false)
    }
    
    pub fn unload_model(&mut self, model_id: &str) -> bool {
        if let Some(ref mut candle_manager) = self.candle_manager {
            if candle_manager.unload_model(model_id) {
                if let Some(status) = self.model_status.get_mut(model_id) {
                    status.loaded = false;
                    status.memory_usage_mb = None;
                }
                return true;
            }
        }
        false
    }
    
    pub fn list_models(&self) -> Vec<&ModelMetadata> {
        self.models.values().collect()
    }
    
    pub fn get_model_status(&self, model_id: &str) -> Option<&ModelStatus> {
        self.model_status.get(model_id)
    }
    
    pub fn get_loaded_models(&self) -> Vec<String> {
        self.model_status
            .iter()
            .filter(|(_, status)| status.loaded)
            .map(|(id, _)| id.clone())
            .collect()
    }
}
