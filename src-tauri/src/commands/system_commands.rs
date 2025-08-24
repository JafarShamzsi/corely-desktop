use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub cpu_count: usize,
    pub total_memory: u64,
    pub available_memory: u64,
    pub os_type: String,
    pub gpu_info: Vec<String>,
}

#[tauri::command]
pub async fn get_system_info() -> Result<SystemInfo, String> {
    use sysinfo::System;
    
    let mut sys = System::new_all();
    sys.refresh_all();
    
    Ok(SystemInfo {
        cpu_count: sys.cpus().len(),
        total_memory: sys.total_memory(),
        available_memory: sys.available_memory(),
        os_type: System::name().unwrap_or_default(),
        gpu_info: vec![], // TODO: Implement GPU detection
    })
}
