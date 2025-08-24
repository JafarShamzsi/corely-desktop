use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLModel {
    pub id: String,
    pub name: String,
    pub framework: String,
    pub size_mb: u64,
    pub is_loaded: bool,
}

#[tauri::command]
pub async fn list_available_models() -> Result<Vec<MLModel>, String> {
    // TODO: Implement model discovery and management
    Ok(vec![
        MLModel {
            id: "sample-model-1".to_string(),
            name: "Sample CNN Model".to_string(),
            framework: "Candle".to_string(),
            size_mb: 250,
            is_loaded: false,
        }
    ])
}

#[tauri::command]
pub async fn load_model(model_id: String) -> Result<bool, String> {
    // TODO: Implement model loading with Candle
    println!("Loading model: {}", model_id);
    Ok(true)
}
